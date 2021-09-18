import java.time.{LocalDate, DayOfWeek}
import java.time.format.DateTimeFormatter

object exer3 {
    def main(args: Array[String]) = {
        println("MergeSort Tests")
        println("MergeSort: " + mergeSort(List(1,9,3,2,7,6,4,8,5)))
        println("MergeSort: " + mergeSort(List(6,2,4,8,9,5,3,1,7,10)))
        println("MergeSort: " + mergeSort(List(4)))
        println("MergeSort: " + mergeSort(List()))

        //How to use Date in Scala: https://www.hackingnote.com/en/scala/datetime
        println("\nIsFriday Tests")
        val date1 = LocalDate.parse("05/17/2018", DateTimeFormatter.ofPattern("MM/dd/yyyy"))
        println("isFriday: " + isFriday(date1))
        val date2 = LocalDate.parse("05/18/2018", DateTimeFormatter.ofPattern("MM/dd/yyyy"))
        println("isFriday: " + isFriday(date2))

        println("\nIsPrimeDay Tests")
        val date3 = LocalDate.parse("05/13/2018", DateTimeFormatter.ofPattern("MM/dd/yyyy"))
        println("isPrimeDay: " + isPrimeDay(date3))
        val date4 = LocalDate.parse("05/14/2018", DateTimeFormatter.ofPattern("MM/dd/yyyy"))
        println("isPrimeDay: " + isPrimeDay(date4))
        val date5 = LocalDate.parse("06/23/2018", DateTimeFormatter.ofPattern("MM/dd/yyyy"))
        println("isPrimeDay: " + isPrimeDay(date5))
    }

    //Pattern Matching (Cases): https://docs.scala-lang.org/tour/pattern-matching.html
    def merge(left: List[Int], right: List[Int]): List[Int] = (left, right) match {
        case (_, List()) => left
        case (List(), _) => right
        case (a :: a1st, b :: b1st) =>
            if (a < b) a :: merge(a1st, right)
            else b :: merge(left, b1st)
    }

    //If else instead of matches (cases), cases cannot have mathematical equations
    def mergeSort(list: List[Int]): List[Int] = {
        if (list.length/2 == 0) {
            list
        }
        else {
            val (left, right) = list splitAt (list.length/2)
            merge(mergeSort(left), mergeSort(right))
        }
    }

    //DayOfWeek Reference (Similar to Java): https://www.geeksforgeeks.org/localdate-getdayofweek-method-in-java/
    def isFriday(date: LocalDate): Boolean = {
        if (date.getDayOfWeek == DayOfWeek.FRIDAY) {
            return true
        }
        else {
            return false
        }
    }

    //divisors() brought from exer2 for isPrimeDay() function
    def divisors(number: Int): List[Int] = {
        for (i <- List.range(2, (number/2)+1) if number % i == 0) yield i
    }

    def isPrimeDay(date: LocalDate): Boolean = {
        if (divisors(date.getDayOfMonth()) == List()) {
            return true
        } 
        else {
            return false
        }
    }
}