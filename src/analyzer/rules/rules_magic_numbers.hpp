#pragma once

#include "../analyzer.hpp"

namespace analyzer {

class MagicNumbersRule : public AnalysisRule {
   public:
    void analyzeProgram(const ast::Program& program,
                        std::vector<Issue>& issues) override;
    std::string getName() const override { return "magic-numbers"; }

   private:
    void checkExpressionForMagicNumbers(const ast::Expr* expr,
                                        std::vector<Issue>& issues,
                                        const std::string& context);
    void checkStatementForMagicNumbers(const ast::Stmt* stmt,
                                       std::vector<Issue>& issues,
                                       const std::string& context);
    void checkBlockForMagicNumbers(const ast::Block& block,
                                   std::vector<Issue>& issues,
                                   const std::string& context);
    bool isMagicNumber(const ast::IntLiteral& literal);
    bool isMagicNumber(const ast::FloatLiteral& literal);
};

}  // namespace analyzer
