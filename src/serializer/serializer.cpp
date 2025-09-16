#include "serializer.hpp"

#include <fstream>
#include <sstream>
#include <filesystem>

namespace {

// Helper to escape strings for JSON output
static std::string escape(const std::string& s) {
    std::ostringstream out;
    for (char c : s) {
        switch (c) {
            case '"': out << "\\\""; break;
            case '\\': out << "\\\\"; break;
            case '\n': out << "\\n"; break;
            case '\r': out << "\\r"; break;
            case '\t': out << "\\t"; break;
            default: out << c; break;
        }
    }
    return out.str();
}

}

namespace astio {

static void writeTypeJson(std::ostream& os, const ast::Type& t) {
    os << "{\"kind\":\"";
    switch (t.kind) {
        case ast::Type::Kind::PRIMITIVE: os << "PRIMITIVE"; break;
        case ast::Type::Kind::TUPLE: os << "TUPLE"; break;
        case ast::Type::Kind::VEC: os << "VEC"; break;
        case ast::Type::Kind::SET: os << "SET"; break;
        case ast::Type::Kind::MAP: os << "MAP"; break;
        case ast::Type::Kind::FUNCTION: os << "FUNCTION"; break;
        case ast::Type::Kind::UNION: os << "UNION"; break;
        case ast::Type::Kind::OPTION: os << "OPTION"; break;
        case ast::Type::Kind::RESULT: os << "RESULT"; break;
        case ast::Type::Kind::STRUCT: os << "STRUCT"; break;
        case ast::Type::Kind::ENUM: os << "ENUM"; break;
        case ast::Type::Kind::INTERFACE: os << "INTERFACE"; break;
        case ast::Type::Kind::GENERIC: os << "GENERIC"; break;
        case ast::Type::Kind::NAMED: os << "NAMED"; break;
    }
    os << "\",\"name\":\"" << escape(t.name) << "\",\"args\":[";
    for (size_t i = 0; i < t.args.size(); ++i) {
        if (i) os << ",";
        writeTypeJson(os, *t.args[i]);
    }
    os << "]}";
}

static void writeExprKind(std::ostream& os, const ast::Expr& e) {
    // Minimal discriminator using RTTI to keep concerns separate from Node definitions
    if (dynamic_cast<const ast::Identifier*>(&e)) os << "\"Identifier\"";
    else if (dynamic_cast<const ast::IntLiteral*>(&e)) os << "\"IntLiteral\"";
    else if (dynamic_cast<const ast::FloatLiteral*>(&e)) os << "\"FloatLiteral\"";
    else if (dynamic_cast<const ast::StringLiteral*>(&e)) os << "\"StringLiteral\"";
    else if (dynamic_cast<const ast::BoolLiteral*>(&e)) os << "\"BoolLiteral\"";
    else if (dynamic_cast<const ast::VecLiteral*>(&e)) os << "\"VecLiteral\"";
    else if (dynamic_cast<const ast::SetLiteral*>(&e)) os << "\"SetLiteral\"";
    else if (dynamic_cast<const ast::MapLiteral*>(&e)) os << "\"MapLiteral\"";
    else if (dynamic_cast<const ast::TupleLiteral*>(&e)) os << "\"TupleLiteral\"";
    else if (dynamic_cast<const ast::EnumVariant*>(&e)) os << "\"EnumVariant\"";
    else if (dynamic_cast<const ast::CallExpr*>(&e)) os << "\"CallExpr\"";
    else if (dynamic_cast<const ast::MemberExpr*>(&e)) os << "\"MemberExpr\"";
    else if (dynamic_cast<const ast::IndexExpr*>(&e)) os << "\"IndexExpr\"";
    else if (dynamic_cast<const ast::UnaryExpr*>(&e)) os << "\"UnaryExpr\"";
    else if (dynamic_cast<const ast::BinaryExpr*>(&e)) os << "\"BinaryExpr\"";
    else os << "\"Expr\"";
}

static void writeStmtKind(std::ostream& os, const ast::Stmt& s) {
    if (dynamic_cast<const ast::Block*>(&s)) os << "\"Block\"";
    else if (dynamic_cast<const ast::IfStmt*>(&s)) os << "\"IfStmt\"";
    else if (dynamic_cast<const ast::WhileStmt*>(&s)) os << "\"WhileStmt\"";
    else if (dynamic_cast<const ast::ForStmt*>(&s)) os << "\"ForStmt\"";
    else if (dynamic_cast<const ast::MatchStmt*>(&s)) os << "\"MatchStmt\"";
    else if (dynamic_cast<const ast::AssignStmt*>(&s)) os << "\"AssignStmt\"";
    else if (dynamic_cast<const ast::ReturnStmt*>(&s)) os << "\"ReturnStmt\"";
    else os << "\"Stmt\"";
}

static void writeFunctionJson(std::ostream& os, const ast::FunctionDecl& f) {
    os << "{\"kind\":\"FunctionDecl\",\"name\":\"" << escape(f.name) << "\",";
    os << "\"params\":[";
    for (size_t i = 0; i < f.params.size(); ++i) {
        if (i) os << ",";
        os << "{\"name\":\"" << escape(f.params[i].name) << "\",\"type\":";
        if (f.params[i].type) writeTypeJson(os, *f.params[i].type);
        else os << "null";
        os << "}";
    }
    os << "],\"returnType\":";
    if (f.returnType) writeTypeJson(os, *f.returnType); else os << "null";
    os << "}";
}

bool writeAstToFile(const ast::Program& program, const std::string& filePath) {
    try {
        std::filesystem::path p(filePath);
        if (p.has_parent_path()) {
            std::error_code ec;
            std::filesystem::create_directories(p.parent_path(), ec);
            // ignore ec; we'll fail on open if needed
        }
        std::ofstream out(filePath, std::ios::binary);
        if (!out) return false;

        out << "{\"program\":{\"units\":[";
        bool first = true;
        for (const auto& unit : program.units) {
            if (!first) out << ","; else first = false;
            if (auto f = dynamic_cast<ast::FunctionDecl*>(unit.get())) {
                writeFunctionJson(out, *f);
            } else if (auto s = dynamic_cast<ast::StructDecl*>(unit.get())) {
                out << "{\"kind\":\"StructDecl\",\"name\":\"" << escape(s->name) << "\"}";
            } else if (auto e = dynamic_cast<ast::EnumDecl*>(unit.get())) {
                out << "{\"kind\":\"EnumDecl\",\"name\":\"" << escape(e->name) << "\"}";
            } else if (auto i = dynamic_cast<ast::InterfaceDecl*>(unit.get())) {
                out << "{\"kind\":\"InterfaceDecl\",\"name\":\"" << escape(i->name) << "\"}";
            } else if (auto a = dynamic_cast<ast::TypeAlias*>(unit.get())) {
                out << "{\"kind\":\"TypeAlias\",\"name\":\"" << escape(a->name) << "\"}";
            } else if (auto impl = dynamic_cast<ast::ImplBlock*>(unit.get())) {
                out << "{\"kind\":\"ImplBlock\",\"type\":\"" << escape(impl->typeName) << "\"}";
            } else {
                out << "{\"kind\":\"Unknown\"}";
            }
        }
        out << "]}}";
        out.flush();
        return static_cast<bool>(out);
    } catch (...) {
        return false;
    }
}

} // namespace astio


