use should::*;

#[test]
fn example1() {
    let points = 10;

    points.should_be(&points);
    points.should_not_be(&50);

    points.should_be_gt(&0);
    points.should_be_lt(&100);

    points.should_satisfy(
        |&x| x * x == 100,
        Some("Expected the square to be 100.".to_string()),
    );
}

#[test]
fn example2() {
    let mut fruits = vec!["apple", "banana", "cherry"];

    fruits.iter().should_be_size(3);

    let dragonfruit = "dragonfruit";
    fruits.push(dragonfruit);

    fruits.iter().should_contain(&&dragonfruit);

    fruits.iter().should_all_satisfy(
        |&x| !x.is_empty(),
        Some("None of the elements should be empty".to_string()),
    );
}
