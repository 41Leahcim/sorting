// contains Stopwatch for measuring performance
using System.Diagnostics;

// create the array for the data, and the stopwatch for measuring performance
double[] values = new double[100000];
Stopwatch stopwatch = new Stopwatch();

// generate the data
Console.WriteLine("Generating data...");
for(int i = 0;i < values.Length;i++){
    values[i] = values.Length - i;
}

// sort the data, measuring the performance
stopwatch.Start();
Console.WriteLine("Sorting data...");
Sort.selection_sort(values);
stopwatch.Stop();

// print the data
Console.WriteLine("Printing data...");
foreach(int value in values){
    Console.WriteLine(value);
}

// print the performance
Console.WriteLine(stopwatch.Elapsed);
