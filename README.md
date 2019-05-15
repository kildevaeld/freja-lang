# Freja Lang


```freja

class CommaFormatter {

    fn fmt(greet, who) {
        return greet + ", " + who + "!"
    }

}

class Greeter extends CommaFormatter {

    init(greeting) {
        this.greeting = greeting
    }

    fn greet(who) {
        return this.fmt(this.greeting, who)
    }

}

let greeter = Greeter("Buongiorno")
let greeting = greeter.greet("mondo")

println(greeting)

```