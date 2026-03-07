<?php

it('throws on empty name', function () {
    new User("");
})->toThrow(\InvalidArgumentException::class);

it('creates a user', function () {
    $user = new User("alice");
    expect($user->getName())->toBe("alice");
});
