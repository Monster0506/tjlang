#include "rules_unused_params.hpp"

#include <iostream>
#include <unordered_set>

namespace analyzer {

static void collectIdentifiers(const ast::Expr* expr, std::unordered_set<std::string>& idents) {
    if (!expr) return;
    if (auto id = dynamic_cast<const ast::Identifier*>(expr)) {
        std::cout << "[COLLECT_IDENTS] Found identifier: " << id->name << std::endl;
        idents.insert(id->name);
        return;
    }
    if (auto bin = dynamic_cast<const ast::BinaryExpr*>(expr)) {
        collectIdentifiers(bin->lhs.get(), idents);
        collectIdentifiers(bin->rhs.get(), idents);
        return;
    }
    if (auto un = dynamic_cast<const ast::UnaryExpr*>(expr)) {
        collectIdentifiers(un->expr.get(), idents);
        return;
    }
    if (auto call = dynamic_cast<const ast::CallExpr*>(expr)) {
        collectIdentifiers(call->callee.get(), idents);
        for (auto& a : call->args) collectIdentifiers(a.get(), idents);
        return;
    }
    if (auto mem = dynamic_cast<const ast::MemberExpr*>(expr)) {
        collectIdentifiers(mem->object.get(), idents);
        return;
    }
    if (auto idx = dynamic_cast<const ast::IndexExpr*>(expr)) {
        collectIdentifiers(idx->object.get(), idents);
        collectIdentifiers(idx->index.get(), idents);
        return;
    }
    if (auto tup = dynamic_cast<const ast::TupleLiteral*>(expr)) {
        for (auto& e : tup->elements) collectIdentifiers(e.get(), idents);
        return;
    }
    if (auto vec = dynamic_cast<const ast::VecLiteral*>(expr)) {
        for (auto& e : vec->elements) collectIdentifiers(e.get(), idents);
        return;
    }
    if (auto set = dynamic_cast<const ast::SetLiteral*>(expr)) {
        for (auto& e : set->elements) collectIdentifiers(e.get(), idents);
        return;
    }
    if (auto map = dynamic_cast<const ast::MapLiteral*>(expr)) {
        for (auto& kv : map->entries) { collectIdentifiers(kv.first.get(), idents); collectIdentifiers(kv.second.get(), idents);}        
        return;
    }
}

static void collectIdentifiersFromBlock(const ast::Block* block, std::unordered_set<std::string>& idents) {
    if (!block) return;
    std::cout << "[COLLECT_FROM_BLOCK] Processing block with " << block->stmts.size() << " statements" << std::endl;
    for (auto& s : block->stmts) {
        // Call the Stmt version of collectIdentifiers
        if (auto ret = dynamic_cast<const ast::ReturnStmt*>(s.get())) {
            std::cout << "[COLLECT_FROM_BLOCK] Found ReturnStmt" << std::endl;
            collectIdentifiers(ret->value.get(), idents);
        } else if (auto asg = dynamic_cast<const ast::AssignStmt*>(s.get())) {
            idents.insert(asg->name);
            collectIdentifiers(asg->value.get(), idents);
        } else if (auto blk = dynamic_cast<const ast::Block*>(s.get())) {
            collectIdentifiersFromBlock(blk, idents);
        } else if (auto iff = dynamic_cast<const ast::IfStmt*>(s.get())) {
            std::cout << "[COLLECT_FROM_BLOCK] Found IfStmt" << std::endl;
            collectIdentifiers(iff->condition.get(), idents);
            collectIdentifiersFromBlock(iff->thenBlock.get(), idents);
            for (auto& p : iff->elifBranches) { 
                collectIdentifiers(p.first.get(), idents); 
                collectIdentifiersFromBlock(p.second.get(), idents);
            }        
            collectIdentifiersFromBlock(iff->elseBlock.get(), idents);
        } else if (auto wh = dynamic_cast<const ast::WhileStmt*>(s.get())) {
            collectIdentifiers(wh->condition.get(), idents);
            collectIdentifiersFromBlock(wh->body.get(), idents);
        } else if (auto fr = dynamic_cast<const ast::ForStmt*>(s.get())) {
            idents.insert(fr->variable);
            collectIdentifiers(fr->iterable.get(), idents);
            collectIdentifiersFromBlock(fr->body.get(), idents);
        } else if (auto mt = dynamic_cast<const ast::MatchStmt*>(s.get())) {
            collectIdentifiers(mt->expression.get(), idents);
            for (auto& g : mt->guards) collectIdentifiers(g.get(), idents);
            for (auto& a : mt->arms) collectIdentifiersFromBlock(a.second.get(), idents);
        }
    }
}

static void collectIdentifiers(const ast::Stmt* stmt, std::unordered_set<std::string>& idents) {
    if (!stmt) return;
    if (auto ret = dynamic_cast<const ast::ReturnStmt*>(stmt)) {
        collectIdentifiers(ret->value.get(), idents);
        return;
    }
    if (auto asg = dynamic_cast<const ast::AssignStmt*>(stmt)) {
        idents.insert(asg->name);
        collectIdentifiers(asg->value.get(), idents);
        return;
    }
    if (auto blk = dynamic_cast<const ast::Block*>(stmt)) {
        collectIdentifiersFromBlock(blk, idents);
        return;
    }
    if (auto iff = dynamic_cast<const ast::IfStmt*>(stmt)) {
        collectIdentifiers(iff->condition.get(), idents);
        collectIdentifiersFromBlock(iff->thenBlock.get(), idents);
        for (auto& p : iff->elifBranches) { 
            collectIdentifiers(p.first.get(), idents); 
            collectIdentifiersFromBlock(p.second.get(), idents);
        }        
        collectIdentifiersFromBlock(iff->elseBlock.get(), idents);
        return;
    }
    if (auto wh = dynamic_cast<const ast::WhileStmt*>(stmt)) {
        collectIdentifiers(wh->condition.get(), idents);
        collectIdentifiersFromBlock(wh->body.get(), idents);
        return;
    }
    if (auto fr = dynamic_cast<const ast::ForStmt*>(stmt)) {
        idents.insert(fr->variable);
        collectIdentifiers(fr->iterable.get(), idents);
        collectIdentifiersFromBlock(fr->body.get(), idents);
        return;
    }
    if (auto mt = dynamic_cast<const ast::MatchStmt*>(stmt)) {
        collectIdentifiers(mt->expression.get(), idents);
        for (auto& g : mt->guards) collectIdentifiers(g.get(), idents);
        for (auto& a : mt->arms) collectIdentifiers(a.second.get(), idents);
        return;
    }
}

void UnusedParamsRule::analyzeProgram(const ast::Program& program, std::vector<Issue>& issues) {
    // This rule is simple and does not need internal debug; rely on analyzer's outer debug prints
    for (const auto& unit : program.units) {
        auto fn = dynamic_cast<const ast::FunctionDecl*>(unit.get());
        if (!fn || !fn->body) continue;

        std::unordered_set<std::string> used;
        collectIdentifiersFromBlock(fn->body.get(), used);

        // Debug output
        std::cout << "[UNUSED_PARAMS] Function: " << fn->name << std::endl;
        std::cout << "[UNUSED_PARAMS] Used identifiers: ";
        for (const auto& ident : used) {
            std::cout << ident << " ";
        }
        std::cout << std::endl;
        std::cout << "[UNUSED_PARAMS] Parameters: ";
        for (const auto& p : fn->params) {
            std::cout << p.name << " ";
        }
        std::cout << std::endl;

        for (const auto& p : fn->params) {
            if (used.find(p.name) == used.end()) {
                Issue i;
                i.severity = Issue::Severity::Warning;
                i.rule = "unused-parameter";
                i.message = "Parameter '" + p.name + "' is never used";
                i.location = fn->name; // simple context
                issues.push_back(std::move(i));
            }
        }
    }
}

} // namespace analyzer


