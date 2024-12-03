# Should

An assertion framework focusing on ergonomics with reasonable error messages.

## Example Usage

`Should` does not require any special setup to use other than adding the usage preamble:

```rs
mod tests {
    // Automatically extends all applicable types to have `Should` functions.
    use should::*;

    #[test]
    fn basic_test() {
        let x = 0;
        let y = 10;

        // Assert whatever you'd like to express:
        x.should_be_lt(&y);
        y.should_be_ge(&x);

        let my_str = "Hello, world!";
        my_str.should_contain("world");

        let my_vec = vec![1, 3, 2];
        my_vec.iter().should_be_size(my_vec.len());
        my_vec.iter().should_contain(&&3);

        // This check will fail!
        my_vec.iter().should_be(&[1, 2, 3]);
    }

}
```

The failure of the above test yields the following output:

```
---- basic_test stdout ----
Assertion failed on thread 'basic_test' at /project-path/tests/test_outside.rs:53:5:
'my_vec.iter()' should be [1, 2, 3] but was Iter([1, 3, 2])

Assertion failed:
   0: test_outside::basic_test
             at /project-path/tests/test_outside.rs:53:5
   1: test_outside::basic_test::{{closure}}
             at /project-path/tests/test_outside.rs:37:16
```
