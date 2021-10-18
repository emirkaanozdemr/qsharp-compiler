// Copyright (c) Microsoft Corporation.
// Licensed under the MIT License.

#include "Generators/DefaultProfileGenerator.hpp"
#include "RuleTransformationPass/RulePass.hpp"
#include "Rules/Factory.hpp"
#include "Rules/FactoryConfig.hpp"
#include "Rules/RuleSet.hpp"

#include "Llvm/Llvm.hpp"

#include <iostream>

namespace microsoft
{
namespace quantum
{

    DefaultProfileGenerator::DefaultProfileGenerator()
    {
        registerProfileComponent<RuleTransformationPassConfiguration>(
            "transformation-rules",
            [](RuleTransformationPassConfiguration const& config, IProfileGenerator* ptr, Profile& profile) {
                auto& ret = ptr->modulePassManager();

                // Default optimisation pipeline
                if (config.simplifyPriorTransform())
                {
                    auto&                   pass_builder = ptr->passBuilder();
                    llvm::ModulePassManager pipeline =
                        pass_builder.buildPerModuleDefaultPipeline(ptr->optimisationLevel());
                    ret.addPass(std::move(pipeline));
                }

                // Defining the mapping
                RuleSet rule_set;
                auto    factory =
                    RuleFactory(rule_set, profile.getQubitAllocationManager(), profile.getResultAllocationManager());
                factory.usingConfiguration(ptr->configurationManager().get<FactoryConfiguration>());

                // Creating profile pass
                ret.addPass(RuleTransformationPass(std::move(rule_set), config, &profile));
            });

        registerProfileComponent<LlvmPassesConfiguration>(
            "llvm-passes", [](LlvmPassesConfiguration const& cfg, IProfileGenerator* ptr, Profile&) {
                auto pass_pipeline = cfg.passPipeline();
                if (!pass_pipeline.empty())
                {

                    auto& pass_builder = ptr->passBuilder();
                    auto& npm          = ptr->modulePassManager();
                    if (!pass_builder.parsePassPipeline(npm, pass_pipeline, false, false))
                    {
                        throw std::runtime_error("Failed to set pass pipeline up.");
                    }
                }
                else if (cfg.alwaysInline())
                {
                    auto& ret          = ptr->modulePassManager();
                    auto& pass_builder = ptr->passBuilder();
                    ret.addPass(llvm::AlwaysInlinerPass());

                    auto inliner_pass = pass_builder.buildInlinerPipeline(
                        ptr->optimisationLevel(), llvm::PassBuilder::ThinLTOPhase::None, ptr->debug());
                    ret.addPass(std::move(inliner_pass));
                }
                else if (!cfg.disableDefaultPipeline())
                {
                    auto& mpm = ptr->modulePassManager();

                    // If not explicitly disabled, we fall back to the default LLVM pipeline
                    auto&                   pass_builder = ptr->passBuilder();
                    llvm::ModulePassManager pipeline1 =
                        pass_builder.buildPerModuleDefaultPipeline(ptr->optimisationLevel());
                    mpm.addPass(std::move(pipeline1));

                    llvm::ModulePassManager pipeline2 = pass_builder.buildModuleSimplificationPipeline(
                        ptr->optimisationLevel(), llvm::PassBuilder::ThinLTOPhase::None);
                    mpm.addPass(std::move(pipeline2));
                }
            });
    }

    DefaultProfileGenerator::DefaultProfileGenerator(
        ConfigureFunction const&                   configure,
        RuleTransformationPassConfiguration const& profile_pass_config,
        LlvmPassesConfiguration const&             llvm_config)
    {
        registerProfileComponent<RuleTransformationPassConfiguration>(
            "transformation-rules",
            [configure](RuleTransformationPassConfiguration const& config, IProfileGenerator* ptr, Profile& profile) {
                auto& ret = ptr->modulePassManager();

                // Default optimisation pipeline
                if (config.simplifyPriorTransform())
                {
                    auto&                   pass_builder = ptr->passBuilder();
                    llvm::ModulePassManager pipeline =
                        pass_builder.buildPerModuleDefaultPipeline(ptr->optimisationLevel());
                    ret.addPass(std::move(pipeline));
                }

                // Defining the mapping
                RuleSet rule_set;
                auto    factory =
                    RuleFactory(rule_set, profile.getQubitAllocationManager(), profile.getResultAllocationManager());
                configure(rule_set);

                // Creating profile pass
                ret.addPass(RuleTransformationPass(std::move(rule_set), config, &profile));
            });

        registerProfileComponent<LlvmPassesConfiguration>(
            "llvm-passes", [](LlvmPassesConfiguration const& cfg, IProfileGenerator* ptr, Profile&) {
                auto pass_pipeline = cfg.passPipeline();
                if (!pass_pipeline.empty())
                {

                    auto& pass_builder = ptr->passBuilder();
                    auto& npm          = ptr->modulePassManager();
                    if (!pass_builder.parsePassPipeline(npm, pass_pipeline, false, false))
                    {
                        throw std::runtime_error("Failed to set pass pipeline up.");
                    }
                }
                else if (cfg.alwaysInline())
                {
                    auto& ret          = ptr->modulePassManager();
                    auto& pass_builder = ptr->passBuilder();
                    ret.addPass(llvm::AlwaysInlinerPass());

                    auto inliner_pass = pass_builder.buildInlinerPipeline(
                        ptr->optimisationLevel(), llvm::PassBuilder::ThinLTOPhase::None, ptr->debug());
                    ret.addPass(std::move(inliner_pass));
                }
                else if (!cfg.disableDefaultPipeline())
                {
                    auto& mpm = ptr->modulePassManager();

                    // If not explicitly disabled, we fall back to the default LLVM pipeline
                    auto&                   pass_builder = ptr->passBuilder();
                    llvm::ModulePassManager pipeline1 =
                        pass_builder.buildPerModuleDefaultPipeline(ptr->optimisationLevel());
                    mpm.addPass(std::move(pipeline1));

                    llvm::ModulePassManager pipeline2 = pass_builder.buildModuleSimplificationPipeline(
                        ptr->optimisationLevel(), llvm::PassBuilder::ThinLTOPhase::None);
                    mpm.addPass(std::move(pipeline2));
                }
            });

        configurationManager().setConfig(profile_pass_config);
        configurationManager().setConfig(llvm_config);
    }

    RuleTransformationPassConfiguration const& DefaultProfileGenerator::ruleTransformationConfig() const
    {
        return configurationManager().get<RuleTransformationPassConfiguration>();
    }

    LlvmPassesConfiguration const& DefaultProfileGenerator::llvmPassesConfig() const
    {
        return configurationManager().get<LlvmPassesConfiguration>();
    }

} // namespace quantum
} // namespace microsoft