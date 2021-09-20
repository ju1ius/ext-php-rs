--TEST--
basic array functionality
--FILE--
<?php

use function ExtPhpRs\TestSuite\Types\{
    passthrough_hashmap_string_i32,
    passthrough_vec_i32,
};

$array = passthrough_vec_i32([1, 2, 3]);

var_dump(is_array($array));
var_dump(count($array));
var_dump($array);

$assoc = passthrough_hashmap_string_i32([
    'a' => 1,
    'b' => 2,
    'c' => 3,
]);

var_dump(is_array($assoc));
var_dump(count($assoc));

// Hashmap order not guaranteed
var_dump($assoc['a']);
var_dump($assoc['b']);
var_dump($assoc['c']);
?>
--EXPECT--
bool(true)
int(3)
array(3) {
  [0]=>
  int(1)
  [1]=>
  int(2)
  [2]=>
  int(3)
}
bool(true)
int(3)
int(1)
int(2)
int(3)
