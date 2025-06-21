use crate::*;

impl SharedRequestBuilder {
    pub(crate) fn build_http_request(
        method: &str,
        path: String,
        header_bytes: Vec<u8>,
        body_bytes: Option<Vec<u8>>,
        http_version_str: String,
    ) -> Vec<u8> {
        let request_line_size: usize = method.len() + 1 + path.len() + 1 + http_version_str.len();
        let body_size: usize = body_bytes.as_ref().map_or(0, |b| b.len());
        let total_size: usize = request_line_size + 2 + header_bytes.len() + 2 + body_size;
        let mut request: Vec<u8> = Vec::with_capacity(total_size);
        request.extend_from_slice(method.as_bytes());
        request.push(b' ');
        request.extend_from_slice(path.as_bytes());
        request.push(b' ');
        request.extend_from_slice(http_version_str.as_bytes());
        request.extend_from_slice(HTTP_BR_BYTES);
        request.extend_from_slice(&header_bytes);
        request.extend_from_slice(HTTP_BR_BYTES);
        if let Some(body) = body_bytes {
            request.extend_from_slice(&body);
        }
        request
    }

    pub(crate) fn build_get_request(
        path: String,
        header_bytes: Vec<u8>,
        http_version_str: String,
    ) -> Vec<u8> {
        Self::build_http_request("GET", path, header_bytes, None, http_version_str)
    }

    pub(crate) fn build_post_request(
        path: String,
        header_bytes: Vec<u8>,
        body_bytes: Vec<u8>,
        http_version_str: String,
    ) -> Vec<u8> {
        Self::build_http_request(
            "POST",
            path,
            header_bytes,
            Some(body_bytes),
            http_version_str,
        )
    }
}

impl SharedResponseHandler {
    pub(crate) fn parse_response_headers(
        headers_bytes: &[u8],
        http_version_bytes: &[u8],
        location_sign_key: &[u8],
        content_length: &mut usize,
        redirect_url: &mut Option<Vec<u8>>,
    ) -> Result<(), RequestError> {
        if let Some(status_pos) =
            Self::find_pattern_case_insensitive(headers_bytes, http_version_bytes)
        {
            let status_code_start: usize = status_pos + http_version_bytes.len() + 1;
            let status_code_end: usize = status_code_start + 3;
            if status_code_end <= headers_bytes.len() {
                let status_code: usize =
                    Self::parse_status_code(&headers_bytes[status_code_start..status_code_end]);

                if (300..=399).contains(&status_code) {
                    if let Some(location_pos) =
                        Self::find_pattern_case_insensitive(headers_bytes, location_sign_key)
                    {
                        let start: usize = location_pos + location_sign_key.len();
                        if let Some(end_pos) = Self::find_crlf(headers_bytes, start) {
                            let mut url_vec = Vec::with_capacity(end_pos - start);
                            url_vec.extend_from_slice(&headers_bytes[start..end_pos]);
                            *redirect_url = Some(url_vec);
                        }
                    }
                }
            }
        }
        *content_length = Self::get_content_length(headers_bytes);
        Ok(())
    }

    pub(crate) fn find_pattern_case_insensitive(haystack: &[u8], needle: &[u8]) -> Option<usize> {
        if needle.is_empty() || haystack.len() < needle.len() {
            return None;
        }
        let needle_len: usize = needle.len();
        let search_len: usize = haystack.len() - needle_len + 1;
        let first_needle_lower: u8 = needle[0].to_ascii_lowercase();
        'outer: for i in 0..search_len {
            if haystack[i].to_ascii_lowercase() != first_needle_lower {
                continue;
            }
            for j in 1..needle_len {
                if haystack[i + j].to_ascii_lowercase() != needle[j].to_ascii_lowercase() {
                    continue 'outer;
                }
            }
            return Some(i);
        }
        None
    }

    pub(crate) fn find_crlf(data: &[u8], start: usize) -> Option<usize> {
        let search_data: &[u8] = &data[start..];
        for i in 0..search_data.len().saturating_sub(1) {
            if search_data[i] == b'\r' && search_data[i + 1] == b'\n' {
                return Some(start + i);
            }
        }
        None
    }

    pub(crate) fn find_double_crlf(data: &[u8], start: usize) -> Option<usize> {
        let search_data: &[u8] = &data[start..];
        for i in 0..search_data.len().saturating_sub(3) {
            if search_data[i] == b'\r'
                && search_data[i + 1] == b'\n'
                && search_data[i + 2] == b'\r'
                && search_data[i + 3] == b'\n'
            {
                return Some(start + i);
            }
        }
        None
    }

    pub(crate) fn get_content_length(response_bytes: &[u8]) -> usize {
        if let Some(pos) =
            Self::find_pattern_case_insensitive(response_bytes, CONTENT_LENGTH_PATTERN)
        {
            let value_start: usize = pos + CONTENT_LENGTH_PATTERN.len();
            let value_start: usize = if response_bytes.get(value_start) == Some(&b' ') {
                value_start + 1
            } else {
                value_start
            };
            if let Some(end_pos) = Self::find_crlf(response_bytes, value_start) {
                let value_bytes: &[u8] = &response_bytes[value_start..end_pos];
                return Self::parse_decimal_bytes(value_bytes);
            }
        }
        0
    }

    pub(crate) fn parse_decimal_bytes(bytes: &[u8]) -> usize {
        let mut result: usize = 0;
        let mut started: bool = false;
        for &byte in bytes {
            match byte {
                b'0'..=b'9' => {
                    started = true;
                    result = result * 10 + (byte - b'0') as usize;
                }
                b' ' | b'\t' if !started => continue,
                _ => break,
            }
        }
        result
    }

    pub(crate) fn parse_status_code(status_bytes: &[u8]) -> usize {
        if status_bytes.len() != 3 {
            return 0;
        }
        let mut result: usize = 0;
        for &byte in status_bytes {
            if byte >= b'0' && byte <= b'9' {
                result = result * 10 + (byte - b'0') as usize;
            } else {
                return 0;
            }
        }
        result
    }

    pub(crate) fn calculate_buffer_capacity(
        response_bytes: &[u8],
        n: usize,
        current_capacity: usize,
    ) -> usize {
        if response_bytes.len() + n <= current_capacity {
            return 0;
        }

        let needed_cap: usize = response_bytes.len() + n;
        if current_capacity == 0 {
            needed_cap.max(1024)
        } else if needed_cap <= current_capacity * 2 {
            current_capacity * 2
        } else {
            (needed_cap * 3) / 2
        }
    }
}
