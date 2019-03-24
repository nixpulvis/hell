# We renamed functions containing '-' because the '-' char is not a
# valid character in Ruby function names. The spec didn't say anything about
# needing a language with '-' in the set of valid characters for
# idents.
# Some other issues with the naming that may cause confusion:
# - name implies a string value, but it is a number
# - start-round and start-turn do not describe the player's action
#

# limitations:
# - A Deck needs to give other ways of looking at the Stacks besides
#   fewest_bulls.
# - There is game state missing from the API. There is no way to keep track of
#   other player's played cards each round, and the choices other players make.

class Player
    attr_reader :name

    def initialize(n)
        @name = n
    end

    def start_round(cards)
        @hand = cards.sort { |a, b| a.face <=> b.face }
    end

    def start_turn(deck)
        @hand.pop
    end

    def choose(deck)
        deck.fewest_bulls
    end
end
