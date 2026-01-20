//! Duplicate file detection orchestration

use crate::hasher::HashAlgorithm;
use dragonfly_core::domain::entities::FileEntity;
use dragonfly_core::domain::value_objects::FilePath;
use dragonfly_core::error::Result;
use jwalk::WalkDir;
use rayon::prelude::*;
use std::collections::HashMap;
use std::path::Path;

/// Duplicate detector orchestrates finding duplicate files
#[derive(Debug, Clone, Copy)]
pub struct DuplicateDetector {
    /// Hash algorithm to use
    algorithm: HashAlgorithm,
}

/// Result of duplicate detection
#[derive(Debug, Clone)]
pub struct DuplicateResult {
    /// Groups of duplicate files (each group contains files with same hash)
    pub duplicates: Vec<Vec<FileEntity>>,
    /// Total space that could be saved by removing duplicates
    pub potential_savings: u64,
}

impl DuplicateDetector {
    /// Create a new duplicate detector with default algorithm (Blake3)
    pub fn new() -> Self {
        Self {
            algorithm: HashAlgorithm::default(),
        }
    }

    /// Create a new duplicate detector with specified algorithm
    pub fn with_algorithm(algorithm: HashAlgorithm) -> Self {
        Self { algorithm }
    }

    /// Find duplicates in a directory
    pub async fn find_duplicates(&self, path: &FilePath, min_size: u64) -> Result<DuplicateResult> {
        let path_str = path.as_str();
        let base_path = Path::new(path_str);

        if !base_path.exists() {
            return Err(dragonfly_core::error::Error::NotFound(format!(
                "Path does not exist: {}",
                path_str
            )));
        }

        // Collect files meeting minimum size
        let files: Vec<FileEntity> = WalkDir::new(base_path)
            .into_iter()
            .par_bridge()
            .filter_map(|entry| {
                let entry = entry.ok()?;
                let metadata = entry.metadata().ok()?;

                if metadata.is_file() && metadata.len() >= min_size {
                    let size = metadata.len();
                    let path_str = entry.path().to_string_lossy().to_string();
                    Some(FileEntity {
                        path: path_str,
                        size,
                    })
                } else {
                    None
                }
            })
            .collect();

        // Group files by hash
        let mut hash_groups: HashMap<String, Vec<FileEntity>> = HashMap::new();

        for file in files {
            let hash = self.compute_hash(&file.path)?;
            hash_groups.entry(hash).or_insert_with(Vec::new).push(file);
        }

        // Filter to only groups with duplicates (2+ files)
        let duplicates: Vec<Vec<FileEntity>> = hash_groups
            .into_values()
            .filter(|group| group.len() > 1)
            .collect();

        // Calculate potential savings (sum of sizes minus one file per group)
        let potential_savings: u64 = duplicates
            .iter()
            .map(|group| {
                let total_size: u64 = group.iter().map(|f| f.size).sum();
                let keep_one = group.first().map(|f| f.size).unwrap_or(0);
                total_size - keep_one
            })
            .sum();

        Ok(DuplicateResult {
            duplicates,
            potential_savings,
        })
    }

    /// Calculate potential space savings from duplicate groups
    pub fn calculate_savings(duplicates: &[Vec<FileEntity>]) -> u64 {
        duplicates
            .iter()
            .map(|group| {
                let total_size: u64 = group.iter().map(|f| f.size).sum();
                let keep_one = group.first().map(|f| f.size).unwrap_or(0);
                total_size - keep_one
            })
            .sum()
    }

    /// Compute hash for a file
    fn compute_hash(&self, file_path: &str) -> Result<String> {
        use std::fs::File;
        use std::io::Read;

        let mut file = File::open(file_path)?;
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer)?;

        let hash = match self.algorithm {
            HashAlgorithm::Blake3 => {
                let mut hasher = blake3::Hasher::new();
                hasher.update(&buffer);
                hasher.finalize().to_hex().to_string()
            }
            HashAlgorithm::XxHash3 => {
                use xxhash_rust::xxh3::Xxh3;
                let mut hasher = Xxh3::new();
                hasher.update(&buffer);
                format!("{:x}", hasher.digest())
            }
        };

        Ok(hash)
    }
}

impl Default for DuplicateDetector {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::io::Write;
    use tempfile::TempDir;

    fn create_test_file(dir: &Path, name: &str, content: &[u8]) -> std::io::Result<String> {
        let file_path = dir.join(name);
        let mut file = fs::File::create(&file_path)?;
        file.write_all(content)?;
        Ok(file_path.to_string_lossy().to_string())
    }

    #[tokio::test]
    async fn should_find_duplicate_files_with_same_content() {
        let temp_dir = TempDir::new().unwrap();
        let test_content = b"identical content";

        let file1 = create_test_file(temp_dir.path(), "file1.txt", test_content).unwrap();
        let file2 = create_test_file(temp_dir.path(), "file2.txt", test_content).unwrap();
        let file3 = create_test_file(temp_dir.path(), "file3.txt", b"different content").unwrap();

        let detector = DuplicateDetector::new();
        let path = FilePath::new(temp_dir.path().to_string_lossy().to_string());
        let result = detector.find_duplicates(&path, 0).await.unwrap();

        assert_eq!(result.duplicates.len(), 1);
        assert_eq!(result.duplicates[0].len(), 2);
        assert!(result.duplicates[0].iter().any(|f| f.path == file1));
        assert!(result.duplicates[0].iter().any(|f| f.path == file2));
        assert!(!result.duplicates[0].iter().any(|f| f.path == file3));
    }

    #[tokio::test]
    async fn should_not_find_duplicates_when_files_differ() {
        let temp_dir = TempDir::new().unwrap();

        create_test_file(temp_dir.path(), "file1.txt", b"content one").unwrap();
        create_test_file(temp_dir.path(), "file2.txt", b"content two").unwrap();
        create_test_file(temp_dir.path(), "file3.txt", b"content three").unwrap();

        let detector = DuplicateDetector::new();
        let path = FilePath::new(temp_dir.path().to_string_lossy().to_string());
        let result = detector.find_duplicates(&path, 0).await.unwrap();

        assert_eq!(result.duplicates.len(), 0);
        assert_eq!(result.potential_savings, 0);
    }

    #[tokio::test]
    async fn should_group_multiple_duplicate_sets() {
        let temp_dir = TempDir::new().unwrap();
        let content_a = b"content A";
        let content_b = b"content B";

        create_test_file(temp_dir.path(), "a1.txt", content_a).unwrap();
        create_test_file(temp_dir.path(), "a2.txt", content_a).unwrap();
        create_test_file(temp_dir.path(), "b1.txt", content_b).unwrap();
        create_test_file(temp_dir.path(), "b2.txt", content_b).unwrap();
        create_test_file(temp_dir.path(), "b3.txt", content_b).unwrap();

        let detector = DuplicateDetector::new();
        let path = FilePath::new(temp_dir.path().to_string_lossy().to_string());
        let result = detector.find_duplicates(&path, 0).await.unwrap();

        assert_eq!(result.duplicates.len(), 2);
        assert_eq!(result.duplicates[0].len(), 2); // Group A
        assert_eq!(result.duplicates[1].len(), 3); // Group B (or vice versa)
    }

    #[tokio::test]
    async fn should_filter_by_minimum_size() {
        let temp_dir = TempDir::new().unwrap();
        let large_content = vec![0u8; 1024]; // 1KB
        let small_content = b"tiny"; // 4 bytes

        create_test_file(temp_dir.path(), "large1.txt", &large_content).unwrap();
        create_test_file(temp_dir.path(), "large2.txt", &large_content).unwrap();
        create_test_file(temp_dir.path(), "small1.txt", small_content).unwrap();
        create_test_file(temp_dir.path(), "small2.txt", small_content).unwrap();

        let detector = DuplicateDetector::new();
        let path = FilePath::new(temp_dir.path().to_string_lossy().to_string());

        // With min_size = 100 bytes, should only find large duplicates
        let result = detector.find_duplicates(&path, 100).await.unwrap();
        assert_eq!(result.duplicates.len(), 1);
        assert_eq!(result.duplicates[0].len(), 2);

        // With min_size = 0, should find both
        let result_all = detector.find_duplicates(&path, 0).await.unwrap();
        assert_eq!(result_all.duplicates.len(), 2);
    }

    #[tokio::test]
    async fn should_calculate_correct_space_savings() {
        let temp_dir = TempDir::new().unwrap();
        let content = vec![0u8; 1000]; // 1KB per file

        create_test_file(temp_dir.path(), "dup1.txt", &content).unwrap();
        create_test_file(temp_dir.path(), "dup2.txt", &content).unwrap();
        create_test_file(temp_dir.path(), "dup3.txt", &content).unwrap();

        let detector = DuplicateDetector::new();
        let path = FilePath::new(temp_dir.path().to_string_lossy().to_string());
        let result = detector.find_duplicates(&path, 0).await.unwrap();

        // 3 files of 1000 bytes each = 3000 total
        // Keep 1 file = 1000 bytes
        // Savings = 2000 bytes
        assert_eq!(result.potential_savings, 2000);
    }

    #[tokio::test]
    async fn should_return_error_for_nonexistent_path() {
        let detector = DuplicateDetector::new();
        let path = FilePath::new("/nonexistent/path/12345".to_string());

        let result = detector.find_duplicates(&path, 0).await;
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            dragonfly_core::error::Error::NotFound(_)
        ));
    }

    #[test]
    fn should_calculate_savings_correctly() {
        let duplicates = vec![
            vec![
                FileEntity {
                    path: "file1.txt".to_string(),
                    size: 1000,
                },
                FileEntity {
                    path: "file2.txt".to_string(),
                    size: 1000,
                },
            ],
            vec![
                FileEntity {
                    path: "file3.txt".to_string(),
                    size: 500,
                },
                FileEntity {
                    path: "file4.txt".to_string(),
                    size: 500,
                },
                FileEntity {
                    path: "file5.txt".to_string(),
                    size: 500,
                },
            ],
        ];

        // Group 1: 2000 total - 1000 keep = 1000 savings
        // Group 2: 1500 total - 500 keep = 1000 savings
        // Total: 2000 savings
        assert_eq!(DuplicateDetector::calculate_savings(&duplicates), 2000);
    }

    #[test]
    fn test_detector_creation() {
        let detector = DuplicateDetector::new();
        assert_eq!(detector.algorithm, HashAlgorithm::Blake3);
    }

    #[test]
    fn test_detector_with_algorithm() {
        let detector = DuplicateDetector::with_algorithm(HashAlgorithm::XxHash3);
        assert_eq!(detector.algorithm, HashAlgorithm::XxHash3);
    }
}
