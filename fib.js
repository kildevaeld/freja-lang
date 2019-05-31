function fib(num) {
    if (num <= 1) {
        return 1;
    }
    return fib(num - 1) + fib(num - 2);
}

var looping = function (n) {
    var a = 0, b = 1, f = 1;
    for (var i = 2; i <= n; i++) {
        f = a + b;
        a = b;
        b = f;
    }
    return f;
};

console.log(looping(85 + 1));