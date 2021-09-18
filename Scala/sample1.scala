object sample1 {
    def main(args: Array[String]) = {
        println(findCharNum("aaabbc"))
        println(findCharNum("How many of each character?"))
        println(findCharNum("Special Characters: !@#$%^&*()"))
        println(findCharNum("Numbers: 1234567890"))
    }

    //Find how many of each characters exists using foldLeft and pattern matching
    //Uppercase & Lowercase do not matter, both count as the same character
    //Spaces also counted as a separate character 
    def findCharNum(sentence: String) = {
        val countChar: Map[Char, Int] = sentence.toLowerCase().foldLeft(Map[Char, Int]()) {
            case (map, char) =>
                val count: Int = map.getOrElse(char, 0)
                map.+(char -> (count + 1))
        }
        countChar
    }
}