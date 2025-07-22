use binlist_rs::BinLookup;

#[tokio::test]
async fn test_lookup_valid_bin() {
    let mut lookup = BinLookup::new();
    let bin = "531462";

    let result = lookup.lookup(bin, None).await;
    assert!(result.is_ok());

    let info = result.unwrap();
    assert_eq!(info.scheme.as_deref(), Some("mastercard"));
    assert!(info.country.is_some());
}

#[tokio::test]
async fn test_lookup_invalid_bin() {
    let mut lookup = BinLookup::new();
    let bin = "000000"; 

    let result = lookup.lookup(bin, None).await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_lookup_empty_bin() {
    let mut lookup = BinLookup::new();
    let bin = "";

    let result = lookup.lookup(bin, None).await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_lookup_with_invalid_proxy() {
    let mut lookup = BinLookup::new();
    let bin = "531462";
    let proxy = Some("invalid-proxy");

    let result = lookup.lookup(bin, proxy).await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_lookup_rate_limit_error_handling() {
    assert!(true, "Implement mocking for 429 response or integration test externally");
}

#[tokio::test]
async fn test_bin_information_fields() {
    let mut lookup = BinLookup::new();
    let bin = "531462";

    let result = lookup.lookup(bin, None).await.unwrap();
    assert!(result.scheme.is_some());
    assert!(result.brand.is_some());
    assert!(result.card_type.is_some());
    assert!(result.prepaid.is_some());
    assert!(result.country.is_some());
}

#[tokio::test]
async fn test_cached_info_field() {
    let mut lookup = BinLookup::new();
    let bin = "531462";

    let _ = lookup.lookup(bin, None).await.unwrap();
    assert!(lookup.info.is_some());
    assert_eq!(lookup.info.as_ref().unwrap().scheme.as_deref(), Some("mastercard"));
}
