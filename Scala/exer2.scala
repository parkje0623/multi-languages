object exer2 {
    def main(args: Array[String]) = {
        println("Divisors Tests")
        println("Divisors: " + divisors(30))
        println("Divisors: " + divisors(64))
        println("Divisors: " + divisors(127))
        //To test using User Input (uncomment below to use)
        // print("Enter an Positive Integer: ")
        // var div_number = scala.io.StdIn.readInt()
        // println("Divisors: " + divisors(div_number))

        println("\nPrimes Tests")
        println("Primes: " + primes(7))
        println("Primes: " + primes(100))
        //To test using User Input (uncomment below to use)
        // print("Enter an Positive Integer: ")
        // var prime_number = scala.io.StdIn.readInt()
        // println("Divisors: " + primes(prime_number))

        println("\nJoin Tests")
        println("Join: " + join(",", List("one", "two", "three")))
        println("Join: " + join("+", List("1", "2", "3")))
        println("Join: " + join("X", List("abc")))
        println("Join: " + join("X", List()))
        

        println("\nPythagorean Tests")
        println("Pythagorean: " + pythagorean(10))
        println("Pythagorean: " + pythagorean(30))
        //To test using User Input (uncomment below to use)
        // print("Enter an Positive Integer: ")
        // var pyth_number = scala.io.StdIn.readInt()
        // println("Divisors: " + pythagorean(pyth_number))
    }

    // Yield Reference: https://alvinalexander.com/scala/scala-for-loop-yield-examples-yield-tutorial/
    def divisors(number: Int): List[Int] = {
        for (i <- List.range(2, (number/2)+1) if number % i == 0) yield i
    }

    def primes(number: Int): List[Int] = {
        for (i <- List.range(2, number+1) if divisors(i) == List()) yield i
    }    

    //mkString for Scala: https://www.baeldung.com/scala/join-string-collection
    def join(element: String, list: List[String]): String = {
        list.mkString(element)
    }

    //Multiple variables in a for loop Scala: https://alvinalexander.com/scala/how-to-for-loops-multiple-counters-multi-dimensional-arrays/
    //Work the same as [(a,b,c) | a<-[1..n], b<-[1..n], c<-[1..n]] in Haskell
    def pythagorean(number: Int): List[(Int, Int, Int)] = {
        for (a <- List.range(1, number+1);
             b <- List.range(1, number+1);
             c <- List.range(1, number+1) 
             if Math.pow(a, 2) + Math.pow(b, 2) == Math.pow(c, 2)
             if a < b) yield (a,b,c)
    } 
}