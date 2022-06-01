#ifndef chess_hpp
#define chess_hpp

#pragma once
#include <string>
#include <list>
#include <vector>

namespace chess
{

  const int BOARD_WIDTH = 8;

  enum Piece
  {
    PAWN,
    ROOK,
    KNIGHT,
    BISHOP,
    QUEEN,
    KING
  };
  enum Type
  {
    BLACK,
    WHITE,
    EMPTY
  };

  struct Square
  {
    Type type;
    Piece piece;
  };

  struct Move
  {
    int x1, y1;
    int x2, y2;
  };

  class Chess
  {
  private:
    int round;
    std::vector<std::vector<Square>> board;

  public:
    Chess();
    Type current_turn();
    std::list<Move> list_next_moves();
    void pawn_next_moves(std::list<Move> &, const int &, const int &);
    void rook_next_moves(std::list<Move> &, const int &, const int &);
    void knight_next_moves(std::list<Move> &, const int &, const int &);
    void bishop_next_moves(std::list<Move> &, const int &, const int &);
    void queen_next_moves(std::list<Move> &, const int &, const int &);
    void king_next_moves(std::list<Move> &, const int &, const int &);
  };

} // namespace chess

///////////////////////////////////////////////////////////////////////////////

chess::Chess::Chess() : round(1), board(chess::BOARD_WIDTH)
{
  board[0] = {{WHITE, ROOK}, {WHITE, KNIGHT}, {WHITE, BISHOP}, {WHITE, QUEEN}, {WHITE, KING}, {WHITE, BISHOP}, {WHITE, KNIGHT}, {WHITE, ROOK}};
  board[1] = {{WHITE, PAWN}, {WHITE, PAWN}, {WHITE, PAWN}, {WHITE, PAWN}, {WHITE, PAWN}, {WHITE, PAWN}, {WHITE, PAWN}, {WHITE, PAWN}};
  board[2] = {{EMPTY, PAWN}, {EMPTY, PAWN}, {EMPTY, PAWN}, {EMPTY, PAWN}, {EMPTY, PAWN}, {EMPTY, PAWN}, {EMPTY, PAWN}, {EMPTY, PAWN}};
  board[3] = board[2];
  board[4] = board[2];
  board[5] = board[2];
  board[6] = {{BLACK, PAWN}, {BLACK, PAWN}, {BLACK, PAWN}, {BLACK, PAWN}, {BLACK, PAWN}, {BLACK, PAWN}, {BLACK, PAWN}, {BLACK, PAWN}};
  board[7] = {{BLACK, ROOK}, {BLACK, KNIGHT}, {BLACK, BISHOP}, {BLACK, QUEEN}, {BLACK, KING}, {BLACK, BISHOP}, {BLACK, KNIGHT}, {BLACK, ROOK}};
}

chess::Type chess::Chess::current_turn() { return round % 2 ? WHITE : BLACK; }

std::list<chess::Move> chess::Chess::list_next_moves()
{
  std::list<Move> moves;
  for (int x = 0; x < BOARD_WIDTH; ++x)
  {
    for (int y = 0; y < BOARD_WIDTH; ++y)
    {
      const Square &sqr = board[x][y];
      if (sqr.type != current_turn())
      {
        continue;
      }

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
      }
    }
  }
  return moves;
}

void chess::Chess::pawn_next_moves(std::list<chess::Move> &moves, const int &x, const int &y)
{
  bool is_first_move = x == (current_turn() == WHITE ? 1 : 6);
  Type opponent = current_turn() == WHITE ? WHITE : BLACK;

  if (current_turn() == WHITE)
  {
    if (x + 1 < BOARD_WIDTH && board[x + 1][y].type == EMPTY)
    {
      moves.push_back({x, y, x + 1, y});
      if (is_first_move && x + 2 < BOARD_WIDTH && board[x + 2][y].type == EMPTY)
      {
        moves.push_back({x, y, x + 2, y});
      }
    }
    if (x + 1 < BOARD_WIDTH && y - 1 >= 0 && board[x + 1][y - 1].type == opponent)
    {
      moves.push_back({x, y, x + 1, y - 1});
    }
    if (x + 1 < BOARD_WIDTH && y + 1 < BOARD_WIDTH && board[x + 1][y + 1].type == opponent)
    {
      moves.push_back({x, y, x + 1, y + 1});
    }
  }
  else
  {
    if (x - 1 >= 0 && board[x - 1][y].type == EMPTY)
    {
      moves.push_back({x, y, x - 1, y});
      if (is_first_move && x - 2 >= 0 && board[x - 2][y].type == EMPTY)
      {
        moves.push_back({x, y, x - 2, y});
      }
    }
    if (x - 1 >= 0 && y - 1 >= 0 && board[x - 1][y - 1].type == opponent)
    {
      moves.push_back({x, y, x - 1, y - 1});
    }
    if (x - 1 >= 0 && y + 1 < BOARD_WIDTH && board[x - 1][y + 1].type == opponent)
    {
      moves.push_back({x, y, x - 1, y + 1});
    }
  }
}

void chess::Chess::rook_next_moves(std::list<chess::Move> &moves, const int &x, const int &y)
{
  for (int i = x + 1; i < BOARD_WIDTH; ++i)
  {
    if (board[i][y].type != current_turn())
    {
      moves.push_back({x, y, i, y});
    }
    if (board[i][y].type != EMPTY)
    {
      break;
    }
  }
  for (int i = x - 1; i >= 0; --i)
  {
    if (board[i][y].type != current_turn())
    {
      moves.push_back({x, y, i, y});
    }
    if (board[i][y].type != EMPTY)
    {
      break;
    }
  }
  for (int i = y + 1; i < BOARD_WIDTH; ++i)
  {
    if (board[x][i].type != current_turn())
    {
      moves.push_back({x, y, x, i});
    }
    if (board[x][i].type != EMPTY)
    {
      break;
    }
  }
  for (int i = y - 1; i >= 0; --i)
  {
    if (board[x][i].type != current_turn())
    {
      moves.push_back({x, y, x, i});
    }
    if (board[x][i].type != EMPTY)
    {
      break;
    }
  }
}

void chess::Chess::knight_next_moves(std::list<chess::Move> &moves, const int &x, const int &y)
{
  if (x - 2 >= 0 && y - 1 >= 0 && board[x - 2][y - 1].type != current_turn())
  {
    moves.push_back({x, y, x - 2, y - 1});
  }
  if (x - 2 >= 0 && y + 1 < BOARD_WIDTH && board[x - 2][y + 1].type != current_turn())
  {
    moves.push_back({x, y, x - 2, y + 1});
  }
  if (x + 2 < BOARD_WIDTH && y - 1 >= 0 && board[x + 2][y - 1].type != current_turn())
  {
    moves.push_back({x, y, x + 2, y - 1});
  }
  if (x + 2 < BOARD_WIDTH && y + 1 < BOARD_WIDTH && board[x + 2][y + 1].type != current_turn())
  {
    moves.push_back({x, y, x + 2, y + 1});
  }
  if (x - 1 >= 0 && y - 2 >= 0 && board[x - 1][y - 2].type != current_turn())
  {
    moves.push_back({x, y, x - 1, y - 2});
  }
  if (x + 1 < BOARD_WIDTH && y - 2 >= 0 && board[x + 1][y - 2].type != current_turn())
  {
    moves.push_back({x, y, x + 1, y - 2});
  }
  if (x - 1 >= 0 && y + 2 < BOARD_WIDTH && board[x - 1][y + 2].type != current_turn())
  {
    moves.push_back({x, y, x - 1, y + 2});
  }
  if (x + 1 < BOARD_WIDTH && y + 2 < BOARD_WIDTH && board[x + 1][y + 2].type != current_turn())
  {
    moves.push_back({x, y, x + 1, y + 2});
  }
}

void chess::Chess::bishop_next_moves(std::list<chess::Move> &moves, const int &x, const int &y)
{
  for (int i = x + 1, j = y + 1; i < BOARD_WIDTH && j < BOARD_WIDTH; ++i, ++j) {
    if (board[i][j].type != current_turn())
    {
      moves.push_back({x, y, i, j});
    }
    if (board[i][j].type != EMPTY)
    {
      break;
    }
  }

  for (int i = x + 1, j = y - 1; i < BOARD_WIDTH && j >= 0; ++i, --j) {
    if (board[i][j].type != current_turn())
    {
      moves.push_back({x, y, i, j});
    }
    if (board[i][j].type != EMPTY)
    {
      break;
    }
  }

  for (int i = x - 1, j = y + 1; i >= 0 && j < BOARD_WIDTH; --i, ++j) {
    if (board[i][j].type != current_turn())
    {
      moves.push_back({x, y, i, j});
    }
    if (board[i][j].type != EMPTY)
    {
      break;
    }
  }

  for (int i = x - 1, j = y - 1; i >= 0 && j >= 0; --i, --j) {
    if (board[i][j].type != current_turn())
    {
      moves.push_back({x, y, i, j});
    }
    if (board[i][j].type != EMPTY)
    {
      break;
    }
  }
}

void chess::Chess::queen_next_moves(std::list<chess::Move> &moves, const int &x, const int &y)
{
}

void chess::Chess::king_next_moves(std::list<chess::Move> &moves, const int &x, const int &y)
{
  if (x - 1 >= 0 && y - 1 >= 0 && board[x - 1][y - 1].type != current_turn())
  {
    moves.push_back({x, y, x - 1, y - 1});
  }
  if (x - 1 >= 0 && board[x - 1][y].type != current_turn())
  {
    moves.push_back({x, y, x - 1, y});
  }
  if (x - 1 >= 0 && y + 1 < BOARD_WIDTH &&
      board[x - 1][y + 1].type != current_turn())
  {
    moves.push_back({x, y, x - 1, y + 1});
  }
  if (y - 1 >= 0 && board[x][y - 1].type != current_turn())
  {
    moves.push_back({x, y, x, y - 1});
  }
  if (y + 1 < BOARD_WIDTH && board[x][y + 1].type != current_turn())
  {
    moves.push_back({x, y, x, y + 1});
  }
  if (x + 1 < BOARD_WIDTH && y - 1 >= 0 &&
      board[x + 1][y - 1].type != current_turn())
  {
    moves.push_back({x, y, x + 1, y - 1});
  }
  if (x + 1 < BOARD_WIDTH && board[x + 1][y].type != current_turn())
  {
    moves.push_back({x, y, x + 1, y});
  }
  if (x + 1 < BOARD_WIDTH && y + 1 < BOARD_WIDTH &&
      board[x + 1][y + 1].type != current_turn())
  {
    moves.push_back({x, y, x + 1, y + 1});
  }
}

#endif // chess_hpp
