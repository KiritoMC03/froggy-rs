pub fn str_simmilarity(str1: &str, str2: &str) -> f64 {
    if str1.len() == 0 || str2.len() == 0 {
        return 0.0;
    }
    else if str1 == str2 {
        return 1.0;
    }
    
    let dist = levenshtein_distance(str1, str2);
    return 1.0 - dist as f64 / usize::max(str1.len(), str2.len()) as f64;
}

pub fn levenshtein_distance(word1: &str, word2: &str) -> usize {
    let len1 = word1.chars().count();
    let len2 = word2.chars().count();
    let mut matrix = vec![vec![0; len2 + 1]; len1 + 1];

    for i in 0..=len1 {
        matrix[i][0] = i;
    }

    for j in 0..=len2 {
        matrix[0][j] = j;
    }

    for (i, char1) in word1.chars().enumerate() {
        for (j, char2) in word2.chars().enumerate() {
            let cost = if char1 == char2 { 0 } else { 1 };

            matrix[i + 1][j + 1] = (matrix[i][j + 1] + 1)
                .min(matrix[i + 1][j] + 1)
                .min(matrix[i][j] + cost);
        }
    }

    matrix[len1][len2]
}