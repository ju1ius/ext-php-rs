<?php declare(strict_types=1);

namespace ExtPhpRs\Tests\Types;

use PHPUnit\Framework\Assert;
use PHPUnit\Framework\TestCase;
use function ExtPhpRs\TestSuite\Types\{
    passthrough_hashmap_string_i32,
    passthrough_str,
    passthrough_string,
    passthrough_vec_i32,
    passthrough_zval,
};

/**
 * @link project://crates/php-testsuite/src/types/passthrough.rs
 */
final class PassthroughTest extends TestCase
{
    /**
     * @dataProvider passthroughZValProvider
     */
    public function testPassthroughZVal(mixed $input): void
    {
        Assert::assertSame($input, passthrough_zval($input));
    }

    public function passthroughZValProvider(): iterable
    {
        yield 'integer' => [42];
        yield 'float' => [66.6];
        yield 'integer array' => [[1, 2, 3]];
        yield 'float array' => [[1.1, 2.2, 3.3]];
        yield 'stdClass' => [new \stdClass()];
        yield 'anonymous class' => [new class() {}];
        yield from $this->passthroughStringProvider();
        yield from $this->passthroughVecI32Provider();
        yield from $this->passthroughHashMapStringI32Provider();
    }

    /**
     * @dataProvider passthroughStringProvider
     */
    public function testPassthroughString(string $input): void
    {
        Assert::assertSame($input, passthrough_string($input));
    }

    public function passthroughStringProvider(): iterable
    {
        yield ["Hello, world!"];
        // TODO: uncomment the following lines to make test fail.
        //  foreach (range(0x00, 0x7F) as $byte) {
        //      yield [chr($byte)];
        //  }
    }

    /**
     * @dataProvider passthroughVecI32Provider
     */
    public function testPassthroughVecI32(array $input): void
    {
        Assert::assertSame($input, passthrough_vec_i32($input));
    }

    public function passthroughVecI32Provider(): iterable
    {
        yield [[1, 2, 3]];
        yield [[0 => 1, 1 => 2, 2 => 3]];
    }

    /**
     * @dataProvider passthroughHashMapStringI32Provider
     */
    public function testPassthroughHashMapStringI32(array $input): void
    {
        // not using assertSame here since Rust HasMaps don't preserve key insertion order.
        Assert::assertEquals($input, passthrough_hashmap_string_i32($input));
    }

    public function passthroughHashMapStringI32Provider(): iterable
    {
        yield [['a' => 1, 'b' => 2, 'c' => 3]];
    }
}
