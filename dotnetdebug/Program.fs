// For more information see https://aka.ms/fsharp-console-apps
printfn "Hello from F#"

let add x y =
    let r = x + y
    r

let result = add 1 2

printfn "Result: %d" result

let args = System.Environment.GetCommandLineArgs()
printfn "Args: %A" args

let arg1 = args.[1]
