--TEST--
basic string functionality
--FILE--
<?php

use function ExtPhpRs\TestSuite\Types\{
    passthrough_str,
    passthrough_string,
};

var_dump(passthrough_string('hello, world!'));
var_dump(passthrough_str('hello, world!'));
?>
--EXPECT--
string(13) "hello, world!"
string(13) "hello, world!"
