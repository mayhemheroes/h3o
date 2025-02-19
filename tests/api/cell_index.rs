use h3o::{error, CellIndex, Resolution};

#[test]
fn is_neighbor_with() {
    let src = CellIndex::try_from(0x8a1fb46622dffff).expect("src");
    let dst = CellIndex::try_from(0x8a1fb46622d7fff).expect("dst");
    assert_eq!(src.is_neighbor_with(dst), Ok(true));

    assert_eq!(
        src.is_neighbor_with(dst.parent(Resolution::Six).expect("parent")),
        Err(error::ResolutionMismatch)
    );

    let dst = CellIndex::try_from(0x8a1fb4644937fff).expect("dst2");
    assert_eq!(src.is_neighbor_with(dst), Ok(false));
}

#[test]
fn to_local_ij() {
    let anchor = CellIndex::try_from(0x823147fffffffff).expect("anchor");
    let index = CellIndex::try_from(0x8230e7fffffffff).expect("index");
    let result = index.to_local_ij(anchor).expect("localij");

    assert_eq!(result.anchor(), anchor);
    assert_eq!(result.i(), -1);
    assert_eq!(result.j(), -2);

    let result =
        index.to_local_ij(anchor.parent(Resolution::One).expect("parent"));

    assert!(result.is_err());
}

#[test]
fn try_from_str() {
    let result = "8a1fb46622dffff".parse::<CellIndex>();
    let expected = CellIndex::try_from(0x8a1fb46622dffff);
    assert_eq!(result, expected, "valid string");

    let result = "no bueno".parse::<CellIndex>();
    assert!(result.is_err(), "invalid string");
}

// Resolutions are displayed as numerical value.
#[test]
fn display() {
    let index = CellIndex::try_from(0x8a1fb46622dffff).expect("index");

    // Default display is the lower hex one.
    let result = index.to_string();
    let expected = "8a1fb46622dffff".to_owned();
    assert_eq!(result, expected, "default display");

    // Upper hex.
    let result = format!("{index:X}");
    let expected = "8A1FB46622DFFFF".to_owned();
    assert_eq!(result, expected, "upper hex");

    // Octal.
    let result = format!("{index:o}");
    let expected = "42417664314213377777".to_owned();
    assert_eq!(result, expected, "octal");

    // Binary.
    let result = format!("{index:b}");
    let expected =
        "100010100001111110110100011001100010001011011111111111111111"
            .to_owned();
    assert_eq!(result, expected, "binary");
}

#[test]
fn child_position() {
    let index = CellIndex::try_from(0x8a1fb46622dffff).expect("index");

    assert_eq!(index.child_position(Resolution::Eight), Some(24));
    assert_eq!(index.child_position(Resolution::Twelve), None);
}

#[test]
fn child_at() {
    let index = CellIndex::try_from(0x881fb46623fffff).expect("index");

    assert_eq!(
        index.child_at(24, Resolution::Ten),
        CellIndex::try_from(0x8a1fb46622dffff).ok(),
    );
    assert_eq!(index.child_at(24, Resolution::Five), None);
}

#[test]
fn child_position_roundtrip() {
    let res = Resolution::Zero;
    let child = CellIndex::try_from(0x8fc3b0804200001).expect("child");
    let parent = child.parent(res).expect("parent");

    let position = child.child_position(res).expect("position");
    let cell = parent.child_at(position, child.resolution());

    assert_eq!(cell, Some(child));
}

#[test]
fn succ() {
    let index = CellIndex::try_from(0o42417664314213377777).expect("index");
    let expected = CellIndex::try_from(0o42417664314213477777).ok();
    assert_eq!(index.succ(), expected, "base case");

    let index = CellIndex::try_from(0o42417664314213677777).expect("index");
    let expected = CellIndex::try_from(0o42417664314214077777).ok();
    assert_eq!(index.succ(), expected, "single carry");

    let index = CellIndex::try_from(0o42417664314666677777).expect("index");
    let expected = CellIndex::try_from(0o42417664315000077777).ok();
    assert_eq!(index.succ(), expected, "cascade carry");

    let index = CellIndex::try_from(0o42466666666666677777).expect("index");
    let expected = CellIndex::try_from(0o42467000000000077777).ok();
    assert_eq!(index.succ(), expected, "cascade to base cell");

    let index = CellIndex::try_from(0o42571666666666677777).expect("index");
    assert!(index.succ().is_none(), "last");

    let index = CellIndex::try_from(0o42404000000000077777).expect("index");
    let expected = CellIndex::try_from(0o42404000000000277777).ok();
    assert_eq!(index.succ(), expected, "deleted subsequence");

    let index = CellIndex::try_from(0x8009fffffffffff).expect("index");
    let expected = CellIndex::try_from(0x800bfffffffffff).ok();
    assert_eq!(index.succ(), expected, "base cell");
}

#[test]
fn pred() {
    let index = CellIndex::try_from(0o42417664314213477777).expect("index");
    let expected = CellIndex::try_from(0o42417664314213377777).ok();
    assert_eq!(index.pred(), expected, "base case");

    let index = CellIndex::try_from(0o42417664314213077777).expect("index");
    let expected = CellIndex::try_from(0o42417664314212677777).ok();
    assert_eq!(index.pred(), expected, "single carry");

    let index = CellIndex::try_from(0o42417664314000077777).expect("index");
    let expected = CellIndex::try_from(0o42417664313666677777).ok();
    assert_eq!(index.pred(), expected, "cascade carry");

    let index = CellIndex::try_from(0o42500000000000077777).expect("index");
    let expected = CellIndex::try_from(0o42477666666666677777).ok();
    assert_eq!(index.pred(), expected, "cascade to base cell");

    let index = CellIndex::try_from(0o42400000000000077777).expect("index");
    assert!(index.pred().is_none(), "last");

    let index = CellIndex::try_from(0o42404000000000277777).expect("index");
    let expected = CellIndex::try_from(0o42404000000000077777).ok();
    assert_eq!(index.pred(), expected, "deleted subsequence");

    let index = CellIndex::try_from(0x800bfffffffffff).expect("index");
    let expected = CellIndex::try_from(0x8009fffffffffff).ok();
    assert_eq!(index.pred(), expected, "base cell");
}

#[test]
fn first() {
    for resolution in Resolution::range(Resolution::Zero, Resolution::Fifteen) {
        assert!(
            CellIndex::first(resolution).pred().is_none(),
            "res {}",
            resolution
        );
    }
}

#[test]
fn last() {
    for resolution in Resolution::range(Resolution::Zero, Resolution::Fifteen) {
        assert!(
            CellIndex::last(resolution).succ().is_none(),
            "res {}",
            resolution
        );
    }
}
