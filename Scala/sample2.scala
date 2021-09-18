import scala.util.Random

//Random Blackjack Game
object sample2 {
    def main(args: Array[String]) = {
        //52 card deck represented as scores in blackjack game
        val deck: List[Int] = List(1,1,1,1,2,2,2,2,3,3,3,3,4,4,4,4,5,5,5,5,6,6,6,6,7,7,7,7,8,8,8,8,9,9,9,9,
                                    10,10,10,10,10,10,10,10,10,10,10,10)

        //Although it is best practice to use immutable variable val,
        //var was used to show that Scala supports easy access of mutable variables by using var
        var player1 = 0
        while (goOrStop(player1)) {
            player1 = blackjack(player1, deck)
            println("Player 1: " + player1)
        }
        var player2 = 0
        while (goOrStop(player2)) {
            player2 = blackjack(player2, deck)
            println("Player 2: " + player2)
        }
        println("Each player's score is: ")
        println("Player 1: " + player1)
        println("Player 2: " + player2)

        if ((player1 > player2 && player1 < 22) || (player1 < 22 && player2 > 21)) {
            println("Player 1 wins the random blackjack game!")
        }
        else if ((player2 > player1 && player2 < 22) || (player2 < 22 && player1 > 21)) {
            println("Player 2 wins the random blackjack game!")
        }
        else if (player1 == player2) {
            println("Draw!")
        }
        else {
            println("Both Players exceeded score 21, no winner")
        }
    }

    //Randomly choose a number from a deck (52 cards) by using Random library
    def blackjack(currScore: Int, cardDeck: List[Int]) = {
        val random = new Random
        val newScore = currScore + cardDeck(random.nextInt(cardDeck.length))
        newScore
    }

    //Randomly decide whether a player should continue to get more card or stop by using Random library
    def goOrStop(currScore: Int): Boolean = {
        if (currScore < 10) {
           return true 
        } 
        else if (currScore < 15) {
            return math.random < 0.70 //70% chance of returning true
        }
        else if (currScore < 19) {
            return math.random < 0.15 //15% chance of returning true
        }
        else {
            return false
        }
    }
}