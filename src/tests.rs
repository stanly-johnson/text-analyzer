#[test]
fn test_count_word_frequency() {
    use std::collections::HashMap;

    // Create a temporary file with sample text
    let file_path = "test_file.txt";
    let text = "Lorem ipsum dolor sit amet, consectetur adipiscing elit. \
                    Sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. \
                    Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris \
                    nisi ut aliquip ex ea commodo consequat.";
    std::fs::write(file_path, text).expect("Failed to write to file");

    // Call the function to count word frequency
    let result = crate::functions::count_word_frequency(file_path);

    // Expected word frequencies
    let expected_frequencies = vec![
        ("lorem".to_string(), 1),
        ("incididunt".to_string(), 1),
        ("ullamco".to_string(), 1),
    ];

    // Convert expected frequencies to a HashMap for easy comparison
    let expected_frequencies: HashMap<String, usize> = expected_frequencies.into_iter().collect();

    // Assert that the result matches the expected word frequencies
    assert_eq!(result.get("lorem"), expected_frequencies.get("lorem"));
    assert_eq!(
        result.get("incididunt"),
        expected_frequencies.get("incididunt")
    );
    assert_eq!(result.get("ullamco"), expected_frequencies.get("ullamco"));

    // Clean up temporary file
    std::fs::remove_file(file_path).expect("Failed to remove file");
}
