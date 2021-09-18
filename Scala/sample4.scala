import scala.util.Random

//Lottery 6/49
object sample4 {
    def main(args: Array[String]) = {
        //Winning number of lottery 6/49
        val winningNum: List[Int] = List(7, 13, 24, 27, 34, 45)
        println("The winning numbers are: " + winningNum)

        //Randomly draw 6 numbers
        val draw: IndexedSeq[Int] = lottery()
        println("The drawn numbers are: " + draw)

        //Check how many are matching
        val matchedCount: Int = checkWin(winningNum, draw)
        println("Totals of " + matchedCount + " numbers matched")

        if (matchedCount < 2) println("The prize is nothing")
        else if (matchedCount == 2) println("The prize is Free Ticket!")
        else if (matchedCount == 3) println("The prize is $10!")
        else if (matchedCount == 3) println("The prize is $5,000!")
        else if (matchedCount == 3) println("The prize is $500,000!")
        else println("The prize is $5,000,000!")
    }

    //Randomly shuffle numbers from 1 to 49 and take fist 6 numbers
    def lottery(max: Int = 49, numbers: Int = 6): IndexedSeq[Int] = {
        val randomGen = Random.shuffle(1 to max).take(numbers)
        randomGen
    }

    //For each randomly dranw numbers, check if any of them matches with the winning numbers
    //Count is incremented if numbers are matched
    def checkWin(win: List[Int], drawn: IndexedSeq[Int]): Int = {
        var matching = 0
        for (i <- List.range(0, 6)) {
            if (drawn.contains(win(i))) {
                matching += 1
            }
        }

        return matching
    }
}