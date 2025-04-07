pub struct DomainRecords {
    pub pivot: String, // 16 bytes
    
    pub domain_prefix: String,
    pub domain_suffix: String,
    pub domain: String,
    pub records: Vec<String>,
}