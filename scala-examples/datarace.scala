import akka.actor._

object DataRace {
  def main(args: Array[String]): Unit = {
    val system = ActorSystem("DataRace")
    val r = system.actorOf(Props[Reporter], "reporter")
    val c = system.actorOf(Props(classOf[CountEvensVsOdds], r), "counter")
    (1 to 100).foreach { x =>
      c ! x
    }
  }
  class Reporter extends Actor {
    def receive = {
      case Array(evens, odds) => println(s"I saw $evens $odds")
    }
  }
  class CountEvensVsOdds(reporter: ActorRef) extends Actor {
    val state = Array(0, 0)
    def receive = {
      case x: Int =>
        if (x % 2 == 0) {
          state(0) = state(0) + 1
        } else {
          state(1) = state(1) + 1
        }
        if ((state(0) + state(1)) % 10 == 0) {
          // RUNTIME error: sharing mutable state between threads
          reporter ! state
        }
    }
  }
}
