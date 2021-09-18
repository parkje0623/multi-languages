//Superclass
class Company {
    val totalBudget: Int = 10000
    val nextYearBudget: List[Int] = List(12000, 9000)
}

//Subclass 'Budget' inherits superclass 'Company'
class Budget extends Company {
    //Any variable from superclass can be used
    val employees: Double = totalBudget*0.4
    val insurance: Double = totalBudget*0.1
    val equipment: Double = totalBudget*0.1
    val loan: Double = totalBudget*0.4

    def budgetDetails(): Boolean = {
        val random = new scala.util.Random
        val nextYear = nextYearBudget(random.nextInt(nextYearBudget.length))

        if (nextYear < totalBudget) {
            return false
        }
        else {
            return true
        }
    }
}

//One simple example that shows the class inheritance in Scala
//Scala's OOP language allows to use class inheritance (Similar to Java)
object sample3 {
    def main(args: Array[String]) = {
        val budget = new Budget()
        //Printing variable from the class Budget
        println("From the total budget, the employees separate their salary from $" + budget.employees)
        println("From the total budget, the insurance payment is $" + budget.insurance)
        println("From the total budget, the equipment cost is $" + budget.equipment)
        println("From the total budget, the loan payment is $" + budget.loan)

        //Calling function from the class Budget
        if (budget.budgetDetails()) {
            println("The company will not need to process bankruptcy")
        }
        else {
            println("The company is required to process bankruptcy")
        }
    }
}