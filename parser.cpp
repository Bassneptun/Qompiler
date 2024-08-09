#include "parser.hh"
#include "tokenizer.hh"

#include <any>
#include <iostream>
#include <string>

std::vector<int64_t> Node::getIdentifier() { return this->identifier; }
std::string Node::getData() { return this->data; }
std::optional<std::vector<Base *>> Node::getChildren() {
  return this->children;
}
std::optional<Base *> Node::getParent() { return this->parent; }

void Node::setIdentifier(std::vector<int64_t> identifier) {
  this->identifier = identifier;
}
void Node::setData(std::string data) { this->data = data; }
void Node::setChildren(std::vector<Base *> children) {
  this->children = children;
}
void Node::addChild(Base *child) { this->children->push_back(child); }
void Node::setParent(Node *parent) { this->parent = parent; }

ProgramRoot *Parser::parse() {
  ProgramRoot *root = new ProgramRoot();
  for (int i = 0; i < this->line_tokens.size(); i++) {
    std::cout << Tokenizer::split(this->line_tokens[i].line)[1] << "\n";
    if (this->line_tokens[i].type == LType::PROCESS) {
      ProcessNode *node = new ProcessNode(
          this->line_tokens[i].tokens, this->line_tokens[i].line,
          Tokenizer::split(this->line_tokens[i].line)[1].c_str(), root,
          LType::PROCESS);
      root->addNode(node);
    } else if (this->line_tokens[i].type == LType::ALLOC) {
      AllocNode *node =
          new AllocNode(this->line_tokens[i].tokens, this->line_tokens[i].line,
                        Tokenizer::split(this->line_tokens[i].line)[1].c_str(),
                        root, LType::ALLOC);
      root->addNode(node);
    } else if (this->line_tokens[i].type == LType::EXPRESSION) {
      ExpressionNode *node =
          new ExpressionNode(this->line_tokens[i].tokens,
                             Tokenizer::split(this->line_tokens[i].line)[1],
                             root, LType::EXPRESSION);
      root->addNode(node);
    } else {
      throw std::runtime_error("Unknown line type");
    }
  }
  return root;
}

int main(int argc, char *argv[]) {
  Parser parser(argv[1]);
  ProgramRoot *root = parser.parse();
  std::cout << "Parsed: " << argv[1] << "\n";
  for (auto node : root->getNodes()) {
    std::cout << node->getType() << "\n\n";
    if (node->getChildren()) {
      for (int i = 0; i < node->getChildren()->size(); i++) {
        std::cout << (*node->getChildren()).size() << "\n";
        std::cout << std::any_cast<const char *>(
                         (*node->getChildren())[i]->getData_())
                  << "\n";
      }
    }
  }
  return 0;
}
