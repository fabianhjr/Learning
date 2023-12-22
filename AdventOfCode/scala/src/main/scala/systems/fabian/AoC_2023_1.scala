package systems.fabian

import scala.util.Try

object AoC_2023_1 extends App:
  val lines = scala.io.Source.stdin.getLines().toList

  val part1 = lines
    .map(_.collect {
      case c if c.isDigit => c.asDigit
    })
    .map(digits => digits.head * 10 + digits.last)
    .sum

  println(s"Part 1: $part1")

  val part2 = lines
    .map(line =>
      line.tails.collect {
        case t if t.headOption.exists(_.isDigit) => t.head.asDigit

        case s"one$tail"   => 1
        case s"two$tail"   => 2
        case s"three$tail" => 3
        case s"four$tail"  => 4
        case s"five$tail"  => 5
        case s"six$tail"   => 6
        case s"seven$tail" => 7
        case s"eight$tail" => 8
        case s"nine$tail"  => 9
      }.toList
    )
    .map(digits => digits.head * 10 + digits.last)
    .sum

  println(s"Part 2: $part2")
end AoC_2023_1
