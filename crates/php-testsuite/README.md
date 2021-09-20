# ext-php-rs PHP tests suite

An extension module build using ext-php-rs,
used to test and benchmark ext-php-rs from PHP.

## Setup

Run `composer install` inside this directory

```sh
(cd crates/php-testsuite && composer install)
```

## Running the tests

To run the PHPUnit test suite:

```sh
cargo xtask phpunit
```

You can also pass arguments to the phpunit executable, e.g.:

```sh
cargo xtask phpunit -- --verbose --stop-on-failure
```

To run the PHPT test suite:

```sh
cargo xtask phpt
```

You can also pass arguments to the phpt runner, e.g.:

```sh
cargo xtask -- -j$(nproc) -M memcheck
```

## Code coverage

To run the full test suite with coverage, run:

```sh
cargo xtask coverage -f html --open
```

## Writing test cases

You can write test either as PHPUnit test cases, or in the PHPT format.

Please refer to the [PHPUnit documentation](https://phpunit.readthedocs.io/)
and the [PHPT syntax reference](https://qa.php.net/phpt_details.php).

You may want to generate the PHP stubs for the php-testsuite extension:

```sh
(cd crates/php-testsuite && cargo-php stubs)
```

### Naming conventions

All rust items exposed to PHP MUST reside in (or in a sub-namespace of)
the `ExtPhpRs\\TestSuite` namespace, i.e.:

```rust
#[php_function(name = "ExtPhpRs\\TestSuite\\SomeNamespace\\some_function")]
pub fn some_function() {}

#[php_class(name = "ExtPhpRs\\TestSuite\\SomeNamespace\\SomeClass")]
pub struct SomeClass;
```
