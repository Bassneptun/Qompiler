#include "tokenizer.hh"

#include <algorithm>
#include <cstdint>
#include <fstream>
#include <iostream>
#include <iterator>
#include <regex>
#include <set>
#include <string>
#include <vector>

template <typename T> void operator*(std::vector<std::vector<T>> container) {
  for (int i = 0; i < container.size(); i++) {
    for (int j = 0; j < container[i].size(); j++) {
      std::cout << container[i][j] << " ";
    }
    std::cout << std::endl;
  }
  std::cout << std::endl;
}

std::vector<std::string> __tokens = {
    "let", "const", "var", "routine", "gate", "H",  "PX", "PY", "PZ",
    "CNT", "CY",    "ID",  "TOF",     "RX",   "RY", "RZ", "S",  "T",
    "SDG", "TDG",   ":",   "{",       "}",    ",",  "=",  ";"};

std::vector<std::string> Tokenizer::split(std::string line) {
  std::vector<std::string> buffer;
  std::string word = "";
  for (int i = 0; i < line.length(); i++) {
    if (line[i] == ' ') {
      buffer.push_back(word);
      word = "";
    } else {
      word += line[i];
    }
  }
  buffer.push_back(word);
  return buffer;
}

std::vector<std::string> split_(std::string line) {
  std::vector<std::string> buffer;
  std::string word = "";
  int in_brackets = 0;
  for (int i = 0; i < line.length(); i++) {
    if (line[i] == ';') {
      if (in_brackets == 0) {
        buffer.push_back(word);
        word = "";
      }
    } else if (line[i] == '{') {
      in_brackets++;
    } else if (line[i] == '}') {
      in_brackets--;
    } else {
      word += line[i];
    }
  }
  buffer.push_back(word);
  return buffer;
}

std::vector<std::string> getInput(std::string path) {
  std::string buffer;
  std::string line;
  std::ifstream myfile(path);
  if (myfile.is_open()) {
    while (std::getline(myfile, line)) {
      buffer.append(line);
    }
    myfile.close();
  }
  return split(buffer, ';');
}

void Tokenizer::tokenizePure() {
  std::transform(buffer.begin(), buffer.end(),
                 std::back_inserter(this->word_list), this->split);
  std::for_each(word_list.begin(), word_list.end(),
                [&](std::vector<std::string> &words) {
                  this->rm_nl(words);
                  this->rm_empty(words);
                });
  std::for_each(
      word_list.begin(), word_list.end(), [&](std::vector<std::string> words) {
        std::vector<int64_t> tmp;
        std::transform(words.begin(), words.end(), std::back_inserter(tmp),
                       [this](std::string word) {
                         return std::find(__tokens.begin(), __tokens.end(),
                                          word) == __tokens.end()
                                    ? -1
                                    : std::find(__tokens.begin(),
                                                __tokens.end(), word) -
                                          __tokens.begin();
                       });
        this->tokens.push_back(tmp);
      });
}

void Tokenizer::lineComponents() {
  this->tokens = this->rm_empty(this->tokens);
  for (int i = 0; i < this->tokens.size(); i++) {
    LineToken l;
    l.line = buffer[i];
    l.tokens = tokens[i];
    if (tokens[i].front() < 3 && tokens[i].front() != -1) {
      l.type = LType::ALLOC;
    } else if (tokens[i].front() < 5) {
      l.type = LType::PROCESS;
    } else if (tokens[i].front() < 18) {
      l.type = LType::EXPRESSION;
    } else if (tokens[i].front() > 18) {
      l.type = LType::LINE_PART;
    } else {
      throw std::runtime_error("Unknown line type");
    }
    this->line_tokens.push_back(l);
  }
}

void Tokenizer::tokenize_rest() {
  static int rcount = 0, ccount = 0, ncount, refcount = 0;
  std::set<std::string> found;
  for (int i = 0; i < this->tokens.size(); i++) {
    for (int j = 0; j < this->tokens[i].size(); j++) {
      if (this->tokens[i][j] == -1) {
        if (std::regex_match(word_list[i][j], std::regex("\\d+(\\.\\d+)?"))) {
          this->tokens[i][j] = 30 + rcount++;
        } else if (std::regex_match(word_list[i][j], std::regex("\\$\\w+"))) {
          this->tokens[i][j] = 1000 + refcount++;
        } else if (std::regex_match(word_list[i][j], std::regex("\\w+"))) {
          this->tokens[i][j] = 100000 + ncount++;
        } else {
          int incr;
          if (found.find(word_list[i][j]) == found.end()) {
            found.insert(word_list[i][j]);
            incr = 1001;
          } else {
            incr = 1001 +
                   std::distance(found.begin(), found.find(word_list[i][j]));
          }
          this->tokens[i][j] = incr + ccount++;
        }
      }
    }
  }
}

std::vector<std::string> Tokenizer::rm_nl(std::vector<std::string> &lines) {
  std::vector<std::string> ret = lines;
  for (auto r : ret) {
    for (int i = r.length() - 1; i >= 0; i--) {
      if (r[i] == '\n') {
        r.erase(r.begin() + i);
      }
    }
  }
  return ret;
}

std::vector<std::string> Tokenizer::rm_empty(std::vector<std::string> &lines) {
  std::vector<std::string> ret;
  for (auto r : ret) {
    if (r.length() != 0) {
      ret.push_back(r);
    }
  }
  return ret;
}

std::vector<std::vector<int64_t>>
Tokenizer::rm_empty(std::vector<std::vector<int64_t>> lines) {
  lines.erase(
      std::remove_if(lines.begin(), lines.end(),
                     [](std::vector<int64_t> line) { return line.empty(); }));
  return lines;
}

void Tokenizer::dump() {
  for (auto tok : this->line_tokens) {
    std::cout << tok.type << "\n{";
    for (int i = 0; i < tok.tokens.size(); i++) {
      std::cout << tok.tokens[i];
      if (i != tok.tokens.size() - 1) {
        std::cout << ",";
      }
    }
    std::cout << "}\n\"" << tok.line << "\"\n\n";
  }
}

int main(int argc, char *argv[]) {
  Tokenizer tokenizer(argv[1]);
  tokenizer.tokenize();
  tokenizer.dump();
  return 0;
}
