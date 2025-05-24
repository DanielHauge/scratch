// For more information see https://aka.ms/fsharp-console-apps
open BenchmarkDotNet.Running
open BenchmarkDotNet.Configs
open BenchmarkDotNet.Diagnosers


type MyBenches() =


    [<BenchmarkDotNet.Attributes.Benchmark>]
    member _.Method1() =
        let infinite_incremental_sequence = Seq.initInfinite (fun i -> i + 1)
        let first_50000 = Seq.take 50000 infinite_incremental_sequence
        let sum = Seq.sum first_50000
        ()


let config =
    (DefaultConfig.Instance.WithOptions ConfigOptions.DisableOptimizationsValidator).AddDiagnoser
        MemoryDiagnoser.Default

let summary = BenchmarkRunner.Run<MyBenches> config
