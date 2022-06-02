#ifndef chess_hpp
#define chess_hpp

#pragma once
#include <ostream>
#include <string>
#include <list>
#include <vector>
#include <unordered_map>

namespace chess
{
  enum Piece
  {
    NONE,
    PAWN,
    ROOK,
    KNIGHT,
    BISHOP,
    QUEEN,
    KING
  };

  enum Color
  {
    BLACK,
    WHITE,
    EMPTY
  };

  const char ABBR[] = {' ', 'P', 'R', 'N', 'B', 'Q', 'K'};

  struct Square
  {
    Color color;
    Piece piece;
  };

  struct Move
  {
    int x1, y1;
    int x2, y2;
    Piece promotion;
  };

  typedef std::vector<std::vector<Square>> Board;

  class Chess
  {
  private:
    int round;
    bool check;
    Board board;

    void list_next_moves(std::list<Move> &, const int &, const int &);
    void pawn_next_moves(std::list<Move> &, const int &, const int &);
    void rook_next_moves(std::list<Move> &, const int &, const int &);
    void knight_next_moves(std::list<Move> &, const int &, const int &);
    void bishop_next_moves(std::list<Move> &, const int &, const int &);
    void queen_next_moves(std::list<Move> &, const int &, const int &);
    void king_next_moves(std::list<Move> &, const int &, const int &);
    void promotions(std::list<Move> &);
    void check_checkmate(const int &, const int &);

  public:
    static const int BOARD_WIDTH = 8;

    Chess();
    Color current_turn();
    std::list<Move> list_next_moves();
    std::string notate(const Move &);

    const Board &get();
  };

} // namespace chess

///////////////////////////////////////////////////////////////////////////////

chess::Chess::Chess() : round(1), board(BOARD_WIDTH)
{
  board[0] = {{WHITE, ROOK}, {WHITE, KNIGHT}, {WHITE, BISHOP}, {WHITE, QUEEN}, {WHITE, KING}, {WHITE, BISHOP}, {WHITE, KNIGHT}, {WHITE, ROOK}};
  board[1] = {{WHITE, PAWN}, {WHITE, PAWN}, {WHITE, PAWN}, {WHITE, PAWN}, {WHITE, PAWN}, {WHITE, PAWN}, {WHITE, PAWN}, {WHITE, PAWN}};
  board[2] = {{EMPTY, NONE}, {EMPTY, NONE}, {EMPTY, NONE}, {EMPTY, NONE}, {EMPTY, NONE}, {EMPTY, NONE}, {EMPTY, NONE}, {EMPTY, NONE}};
  board[3] = board[2];
  board[4] = board[2];
  board[5] = board[2];
  board[6] = {{BLACK, PAWN}, {BLACK, PAWN}, {BLACK, PAWN}, {BLACK, PAWN}, {BLACK, PAWN}, {BLACK, PAWN}, {BLACK, PAWN}, {BLACK, PAWN}};
  board[7] = {{BLACK, ROOK}, {BLACK, KNIGHT}, {BLACK, BISHOP}, {BLACK, QUEEN}, {BLACK, KING}, {BLACK, BISHOP}, {BLACK, KNIGHT}, {BLACK, ROOK}};
}

chess::Color chess::Chess::current_turn() { return round % 2 ? WHITE : BLACK; }

std::list<chess::Move> chess::Chess::list_next_moves()
{
  std::list<Move> moves;
  for (int x = 0; x < BOARD_WIDTH; ++x)
  {
    for (int y = 0; y < BOARD_WIDTH; ++y)
    {
      if (board[x][y].color != current_turn())
      {
        continue;
      }
      list_next_moves(moves, x, y);
    }
  }
  return moves;
}

void chess::Chess::list_next_moves(std::list<Move> &moves, const int &x, const int &y)
{
  const Square &sqr = board[x][y];
  switch (sqr.piece)
  {
  case PAWN:
    pawn_next_moves(moves, x, y);
    break;
  case ROOK:
    rook_next_moves(moves, x, y);
    break;
  case KNIGHT:
    knight_next_moves(moves, x, y);
    break;
  case BISHOP:
    bishop_next_moves(moves, x, y);
    break;
  case QUEEN:
    queen_next_moves(moves, x, y);
    break;
  case KING:
    king_next_moves(moves, x, y);
    break;
  default:;
  }
}

void chess::Chess::pawn_next_moves(std::list<chess::Move> &moves, const int &x, const int &y)
{
  bool is_first_move = x == (current_turn() == WHITE ? 1 : 6);
  Color opponent = current_turn() == WHITE ? WHITE : BLACK;

  if (current_turn() == WHITE)
  {
    if (x + 1 < BOARD_WIDTH && board[x + 1][y].color == EMPTY)
    {
      moves.push_back({x, y, x + 1, y});
      promotions(moves);
      if (is_first_move && x + 2 < BOARD_WIDTH && board[x + 2][y].color == EMPTY)
      {
        moves.push_back({x, y, x + 2, y});
      }
    }
    if (x + 1 < BOARD_WIDTH && y - 1 >= 0 && board[x + 1][y - 1].color == opponent)
    {
      moves.push_back({x, y, x + 1, y - 1});
      promotions(moves);
    }
    if (x + 1 < BOARD_WIDTH && y + 1 < BOARD_WIDTH && board[x + 1][y + 1].color == opponent)
    {
      moves.push_back({x, y, x + 1, y + 1});
      promotions(moves);
    }
  }
  else
  {
    if (x - 1 >= 0 && board[x - 1][y].color == EMPTY)
    {
      moves.push_back({x, y, x - 1, y});
      promotions(moves);
      if (is_first_move && x - 2 >= 0 && board[x - 2][y].color == EMPTY)
      {
        moves.push_back({x, y, x - 2, y});
      }
    }
    if (x - 1 >= 0 && y - 1 >= 0 && board[x - 1][y - 1].color == opponent)
    {
      moves.push_back({x, y, x - 1, y - 1});
      promotions(moves);
    }
    if (x - 1 >= 0 && y + 1 < BOARD_WIDTH && board[x - 1][y + 1].color == opponent)
    {
      moves.push_back({x, y, x - 1, y + 1});
      promotions(moves);
    }
  }
}

void chess::Chess::rook_next_moves(std::list<chess::Move> &moves, const int &x, const int &y)
{
  for (int i = x + 1; i < BOARD_WIDTH; ++i)
  {
    if (board[i][y].color != current_turn())
    {
      moves.push_back({x, y, i, y});
    }
    if (board[i][y].color != EMPTY)
    {
      break;
    }
  }
  for (int i = x - 1; i >= 0; --i)
  {
    if (board[i][y].color != current_turn())
    {
      moves.push_back({x, y, i, y});
    }
    if (board[i][y].color != EMPTY)
    {
      break;
    }
  }
  for (int i = y + 1; i < BOARD_WIDTH; ++i)
  {
    if (board[x][i].color != current_turn())
    {
      moves.push_back({x, y, x, i});
    }
    if (board[x][i].color != EMPTY)
    {
      break;
    }
  }
  for (int i = y - 1; i >= 0; --i)
  {
    if (board[x][i].color != current_turn())
    {
      moves.push_back({x, y, x, i});
    }
    if (board[x][i].color != EMPTY)
    {
      break;
    }
  }
}

void chess::Chess::knight_next_moves(std::list<chess::Move> &moves, const int &x, const int &y)
{
  if (x - 2 >= 0 && y - 1 >= 0 && board[x - 2][y - 1].color != current_turn())
  {
    moves.push_back({x, y, x - 2, y - 1});
  }
  if (x - 2 >= 0 && y + 1 < BOARD_WIDTH && board[x - 2][y + 1].color != current_turn())
  {
    moves.push_back({x, y, x - 2, y + 1});
  }
  if (x + 2 < BOARD_WIDTH && y - 1 >= 0 && board[x + 2][y - 1].color != current_turn())
  {
    moves.push_back({x, y, x + 2, y - 1});
  }
  if (x + 2 < BOARD_WIDTH && y + 1 < BOARD_WIDTH && board[x + 2][y + 1].color != current_turn())
  {
    moves.push_back({x, y, x + 2, y + 1});
  }
  if (x - 1 >= 0 && y - 2 >= 0 && board[x - 1][y - 2].color != current_turn())
  {
    moves.push_back({x, y, x - 1, y - 2});
  }
  if (x + 1 < BOARD_WIDTH && y - 2 >= 0 && board[x + 1][y - 2].color != current_turn())
  {
    moves.push_back({x, y, x + 1, y - 2});
  }
  if (x - 1 >= 0 && y + 2 < BOARD_WIDTH && board[x - 1][y + 2].color != current_turn())
  {
    moves.push_back({x, y, x - 1, y + 2});
  }
  if (x + 1 < BOARD_WIDTH && y + 2 < BOARD_WIDTH && board[x + 1][y + 2].color != current_turn())
  {
    moves.push_back({x, y, x + 1, y + 2});
  }
}

void chess::Chess::bishop_next_moves(std::list<chess::Move> &moves, const int &x, const int &y)
{
  for (int i = x + 1, j = y + 1; i < BOARD_WIDTH && j < BOARD_WIDTH; ++i, ++j)
  {
    if (board[i][j].color != current_turn())
    {
      moves.push_back({x, y, i, j});
    }
    if (board[i][j].color != EMPTY)
    {
      break;
    }
  }

  for (int i = x + 1, j = y - 1; i < BOARD_WIDTH && j >= 0; ++i, --j)
  {
    if (board[i][j].color != current_turn())
    {
      moves.push_back({x, y, i, j});
    }
    if (board[i][j].color != EMPTY)
    {
      break;
    }
  }

  for (int i = x - 1, j = y + 1; i >= 0 && j < BOARD_WIDTH; --i, ++j)
  {
    if (board[i][j].color != current_turn())
    {
      moves.push_back({x, y, i, j});
    }
    if (board[i][j].color != EMPTY)
    {
      break;
    }
  }

  for (int i = x - 1, j = y - 1; i >= 0 && j >= 0; --i, --j)
  {
    if (board[i][j].color != current_turn())
    {
      moves.push_back({x, y, i, j});
    }
    if (board[i][j].color != EMPTY)
    {
      break;
    }
  }
}

void chess::Chess::queen_next_moves(std::list<chess::Move> &moves, const int &x, const int &y)
{
  rook_next_moves(moves, x, y);
  bishop_next_moves(moves, x, y);
}

void chess::Chess::king_next_moves(std::list<chess::Move> &moves, const int &x, const int &y)
{
  if (x - 1 >= 0 && y - 1 >= 0 && board[x - 1][y - 1].color != current_turn())
  {
    moves.push_back({x, y, x - 1, y - 1});
  }
  if (x - 1 >= 0 && board[x - 1][y].color != current_turn())
  {
    moves.push_back({x, y, x - 1, y});
  }
  if (x - 1 >= 0 && y + 1 < BOARD_WIDTH &&
      board[x - 1][y + 1].color != current_turn())
  {
    moves.push_back({x, y, x - 1, y + 1});
  }
  if (y - 1 >= 0 && board[x][y - 1].color != current_turn())
  {
    moves.push_back({x, y, x, y - 1});
  }
  if (y + 1 < BOARD_WIDTH && board[x][y + 1].color != current_turn())
  {
    moves.push_back({x, y, x, y + 1});
  }
  if (x + 1 < BOARD_WIDTH && y - 1 >= 0 &&
      board[x + 1][y - 1].color != current_turn())
  {
    moves.push_back({x, y, x + 1, y - 1});
  }
  if (x + 1 < BOARD_WIDTH && board[x + 1][y].color != current_turn())
  {
    moves.push_back({x, y, x + 1, y});
  }
  if (x + 1 < BOARD_WIDTH && y + 1 < BOARD_WIDTH &&
      board[x + 1][y + 1].color != current_turn())
  {
    moves.push_back({x, y, x + 1, y + 1});
  }
}

void chess::Chess::promotions(std::list<Move> &moves)
{
  const int &x1 = moves.back().x1;
  const int &y1 = moves.back().y1;
  const int &x2 = moves.back().x2;
  const int &y2 = moves.back().y2;

  if (x2 == (current_turn() == WHITE ? BOARD_WIDTH - 1 : 0))
  {
    moves.back().promotion = QUEEN;
    moves.push_back({x1, y1, x2, y2, BISHOP});
    moves.push_back({x1, y1, x2, y2, ROOK});
    moves.push_back({x1, y1, x2, y2, KNIGHT});
  }
}

void chess::Chess::check_checkmate(const int &x, const int &y)
{
  check = false;
  std::list<Move> moves;
  list_next_moves(moves, x, y);
  for (const Move &move : moves)
  {
    if (board[move.x2][move.y2].piece == KING)
    {
      check = true;
      break;
    }
  }
}

std::string chess::Chess::notate(const Move &move)
{
  const Square &src = board[move.x1][move.y1];
  const Square &dst = board[move.x2][move.y2];

  std::string notation;
  if (src.piece != PAWN)
  {
    notation += ABBR[src.piece];
  }
  if (dst.color != EMPTY)
  {
    if (src.piece == PAWN)
    {
      notation += 'a' + move.y1;
    }
    notation += 'x';
  }
  notation += 'a' + move.y2;
  notation += '1' + move.x2;

  if (src.piece == PAWN && move.x2 == (current_turn() == WHITE ? BOARD_WIDTH - 1 : 0))
  {
    notation += '=';
    notation += ABBR[move.promotion];
  }

  if (check)
  {
    notation += '+';
  }

  return notation;
}

const chess::Board &chess::Chess::get()
{
  return board;
}

///////////////////////////////////////////////////////////////////////////////

std::ostream &operator<<(std::ostream &os, const chess::Board &board)
{
  using namespace chess;

  int rank_num = Chess::BOARD_WIDTH;
  os << "  ---------------------------------\n";
  for (const auto &rank : board)
  {
    os << rank_num-- << ' ';
    for (const Square &sqr : rank)
    {
      os << '|' << ' ' << ABBR[sqr.piece] << ' ';
    }
    os << '|' << '\n';
    os << "  ---------------------------------\n";
  }
  os << "    a   b   c   d   e   f   g   h  \n";
  return os;
}

#endif // chess_hpp
