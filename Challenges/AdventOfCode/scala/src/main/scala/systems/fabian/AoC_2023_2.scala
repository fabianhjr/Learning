package systems.fabian

import scala.collection.mutable.Map as MutableMap

import cats.data.NonEmptyList
import cats.implicits.*
import cats.parse.*

object AoC_2023_2 extends App:
  enum Color:
    case Red, Green, Blue
  end Color

  val space: Parser[Unit] = Parser.char(' ')
  val digits: Parser[Int] = Parser.charsWhile(_.isDigit).map(_.toInt)

  val color: Parser[Color] = Parser.oneOf(
    List(
      Parser.string("red").as(Color.Red),
      Parser.string("green").as(Color.Green),
      Parser.string("blue").as(Color.Blue)
    )
  )

  val colorInfo: Parser[(Color, Int)]    =
    (digits ~ (space *> color)).map(_.swap)
  val roundInfo: Parser[Map[Color, Int]] =
    colorInfo.repSep(Parser.char(',') ~ space).map(_.toList.toMap)

  val gameHeader: Parser[Int]                            =
    Parser.string("Game") *> space *> digits <* Parser.char(':') <* space
  val gameInfo: Parser[NonEmptyList[Map[Color, Int]]]    =
    roundInfo.repSep(Parser.char(';') ~ space)
  val game: Parser[(Int, NonEmptyList[Map[Color, Int]])] = gameHeader ~ gameInfo

  val input = scala.io.Source.stdin.getLines().toList

  val parsed = input.map(game.parseAll).map {
    case Right(r)  => r
    case Left(err) =>
      println(err.show)
      ???
  }

  def checkPossible(red: Int, green: Int, blue: Int)(
      round: Map[Color, Int]
  ): Boolean =
    val roundRed   = round.getOrElse(Color.Red, 0)
    val roundGreen = round.getOrElse(Color.Green, 0)
    val roundBlue  = round.getOrElse(Color.Blue, 0)

    roundRed <= red && roundGreen <= green && roundBlue <= blue

  val part1 = parsed.collect {
    case (id, rounds) if rounds.forall(checkPossible(12, 13, 14)) => id
  }.sum

  println(s"Part 1: $part1")

  val part2 = parsed.map { (id, rounds) =>
    rounds.toList.map(MutableMap.from).fold(MutableMap.empty) { (acc, round) =>
      round.iterator.foreach { case (color, count) =>
        acc.updateWith(color) {
          case None => Some(count)
          case Some(existing) => Some(existing max count)
        }
      }
      acc
    }
  }.map(_.values.product).sum

  println(s"Part 2: $part2")
end AoC_2023_2
