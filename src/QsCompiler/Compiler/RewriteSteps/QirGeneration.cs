﻿using System;
using System.Collections.Generic;
using Microsoft.Quantum.QsCompiler.QirGenerator;
using Microsoft.Quantum.QsCompiler.SyntaxTree;

namespace Microsoft.Quantum.QsCompiler.BuiltInRewriteSteps
{
    public class QirGeneration : IRewriteStep
    {
        private readonly string outputFile;

        private readonly List<IRewriteStep.Diagnostic> diagnostics = new List<IRewriteStep.Diagnostic>();

        public QirGeneration(string outputFileName)
        {
            this.outputFile = outputFileName;
        }

        /// <inheritdoc/>
        public string Name => "QIR Generation";

        /// <inheritdoc/>
        public int Priority => 0;

        /// <inheritdoc/>
        public IDictionary<string, string?> AssemblyConstants { get; } = new Dictionary<string, string?>();

        /// <inheritdoc/>
        public IEnumerable<IRewriteStep.Diagnostic> GeneratedDiagnostics => this.diagnostics;

        /// <inheritdoc/>
        public bool ImplementsPreconditionVerification => false;

        /// <inheritdoc/>
        public bool ImplementsTransformation => false;

        /// <inheritdoc/>
        public bool ImplementsPostconditionVerification => false;

        /// <inheritdoc/>
        public bool PostconditionVerification(QsCompilation compilation)
        {
            return true;
        }

        /// <inheritdoc/>
        public bool PreconditionVerification(QsCompilation compilation)
        {
            return true;
        }

        /// <inheritdoc/>
        public bool Transformation(QsCompilation compilation, out QsCompilation transformed)
        {
            try
            {
                var transformation = new QirTransformation(compilation, new Configuration());
                transformation.Apply();
                transformation.Emit(this.outputFile);
            }
            catch (Exception ex)
            {
                this.diagnostics.Add(new IRewriteStep.Diagnostic() { Severity = CodeAnalysis.DiagnosticSeverity.Warning, Message = ex.Message });
                //File.WriteAllText($"{this.config.OutputFileName}.ll", $"Exception: {ex.Message} at:\n{ex.StackTrace}");
            }

            transformed = compilation;

            return true;
        }
    }
}