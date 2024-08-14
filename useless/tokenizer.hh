#ifndef Qompiler_hh
#define Qompiler_hh

#include <cstdint>
#include <string>
#include <vector>

extern std::vector<std::string> __tokens;

extern std::vector<int> allocs;
extern std::vector<int> expressions;
extern std::vector<int> process;

enum LType { ALLOC, EXPRESSION, PROCESS, LINE_PART, UNKNOWN };

struct LineToken {
  enum LType type;
  std::vector<int64_t> tokens;
  std::string line;
};

std::vector<std::string> split(std::string line, char delim = ' ');

std::vector<std::string> getInput(std::string path);

class Tokenizer {
public:
  Tokenizer(std::string path) : buffer(getInput(path)) {}
  ~Tokenizer() {}

  struct Toks {
    std::vector<std::string> *buffer;
    std::vector<std::vector<int64_t>> *tokens;
    std::vector<LineToken> *line_tokens;
  };

  void dump();

  void tokenizePure();
  void lineComponents();
  void tokenize_rest();

  Toks tokenize() {
    this->tokenizePure();
    this->tokenize_rest();
    this->lineComponents();
    return Toks{&this->buffer, &this->tokens, &this->line_tokens};
  }

  static std::vector<std::vector<int64_t>>
      rm_empty(std::vector<std::vector<int64_t>>);
  static std::vector<std::string> rm_empty(std::vector<std::string> &lines);
  static std::vector<std::string> rm_nl(std::vector<std::string> &lines);
  static std::vector<std::string> split(std::string line);
  std::vector<std::vector<int64_t>> get_tokens() { return this->tokens; }
  std::vector<LineToken> get_line_tokens() { return this->line_tokens; }

private:
  std::vector<std::string> buffer;
  std::vector<std::vector<int64_t>> tokens;
  std::vector<LineToken> line_tokens;
  std::vector<std::vector<std::string>> word_list;
};

#endif // Qompiler_hh
