#ifndef PARSER_hh
#define PARSER_hh

#include <any>
#include <cstdint>
#include <functional>
#include <optional>
#include <string>
#include <variant>
#include <vector>

#include "tokenizer.hh"

constexpr unsigned int djb2(const char *str, unsigned int hash = 5381) {
  return (*str) ? djb2(str + 1,
                       ((hash << 5) + hash) + static_cast<unsigned int>(*str))
                : hash;
}

enum LEAF_TYPE { STRING, INTEGER, DOUBLE, BOOLEAN };
struct LeafReturn {
  std::any data;
  LEAF_TYPE type;
};

class Base {
  std::optional<std::any> _data;
  std::optional<LEAF_TYPE> _type;

public:
  LeafReturn getData_() { return LeafReturn{this->_data, *this->_type}; }
  void setData_(std::any data) { this->_data = data; }
  void setType_(LEAF_TYPE type) { this->_type = type; }
};

class Node : public Base {
public:
  explicit Node()
      : identifier(-2), parent(std::nullopt), children(std::nullopt), Base() {}
  explicit Node(std::vector<int64_t> identifier, std::string data)
      : identifier(identifier), data(data), parent(std::nullopt),
        children(std::nullopt), Base() {}
  explicit Node(std::vector<int64_t> identifier, std::string data, Base *parent)
      : identifier(identifier), data(data), parent(parent),
        children(std::nullopt), Base() {}
  explicit Node(std::string data)
      : identifier(-2), data(data), parent(std::nullopt),
        children(std::nullopt), Base() {}
  explicit Node(std::vector<int64_t> identifier, std::string data, Base *parent,
                LType type)
      : identifier(identifier), data(data), parent(parent),
        children(std::nullopt), type(type), Base() {}

private:
  std::vector<int64_t> identifier;
  std::string data;
  std::optional<std::vector<Base *>> children;
  std::optional<Base *> parent;
  LType type;

public:
  virtual std::vector<int64_t> getIdentifier();
  virtual std::string getData();
  virtual std::optional<std::vector<Base *>> getChildren();
  virtual std::optional<Base *> getParent();
  virtual LType getType() { return this->type; }

  virtual void setIdentifier(std::vector<int64_t> identifier);
  virtual void setData(std::string data);
  virtual void setChildren(std::vector<Base *> children);
  virtual void addChild(Base *child);
  virtual void setParent(Node *parent);
  virtual void setType(LType type) { this->type = type; }
};

class Leaf : public Base {
public:
  explicit Leaf(int64_t identifier, std::any data, LEAF_TYPE type, Node *parent)
      : parent(parent), type(type), identifier(identifier), Base() {
    this->setData_(data);
  }

private:
  int64_t identifier;
  LEAF_TYPE type;
  Node *parent;
};

class AllocNode : public Node {
public:
  explicit AllocNode(std::vector<int64_t> identifier, std::string data,
                     std::string var_name, Base *parent, LType type)
      : Node(identifier, data, parent, type) {
    auto l = new Leaf(-1, var_name, LEAF_TYPE::STRING, this);
    this->addChild(l);
  }
};

class ProcessNode : public Node {
public:
  explicit ProcessNode(std::vector<int64_t> identifier, std::string data,
                       std::string var_name, Base *parent, LType type)
      : Node(identifier, var_name, parent, type) {
    std::vector<std::string> tmp = split(data, '\n');
    for (int i = 0; i < tmp.size(); i++) {
      auto l = new Leaf(-1, var_name, LEAF_TYPE::STRING, this);
      this->addChild(l);
    }
  }
};

class ExpressionNode : public Node {
public:
  explicit ExpressionNode(std::vector<int64_t> identifier,
                          std::string reference, Base *parent, LType type)
      : Node(identifier, reference, parent, type) {
    auto l = new Leaf(-1, reference, LEAF_TYPE::STRING, this);
    this->addChild(l);
  }
};

class ProgramRoot : public Base {
public:
  explicit ProgramRoot() : Base() {}

  void addNode(Node *node) { this->nodes.push_back(node); }
  std::vector<Node *> getNodes() { return this->nodes; }
  void setNodes(std::vector<Node *> nodes) { this->nodes = nodes; }

  void visit(std::function<void(Node *)> f) {
    for (int i = 0; i < this->nodes.size(); i++) {
      f(this->nodes[i]);
    }
  }

private:
  std::vector<Node *> nodes;
};

class Parser {
public:
  Parser(std::string path) : buffer(getInput(path)) {
    Tokenizer tokenizer(path);
    Tokenizer::Toks tmp = tokenizer.tokenize();
    this->tokens = *tmp.tokens;
    this->line_tokens = *tmp.line_tokens;
  }
  ~Parser() {}

  ProgramRoot *parse();
  std::string identifierToString(std::vector<int64_t> identifier);
  bool check(std::vector<int64_t> identifier);

private:
  std::vector<std::string> buffer;
  std::vector<std::vector<int64_t>> tokens;
  std::vector<LineToken> line_tokens;
};

#endif
