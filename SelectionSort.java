// 10: 3.104E-6
// 100: 6.9958E-5
// 1000: 0.002227221
// 10000: 0.117138454
// 100000: 3.740233637
public class SelectionSort {
    public static void sort(double[] values){
        for(int i = 0;i < values.length;i++){
            int lowest_index = i;
            for(int j = i + 1;j < values.length;j++){
                if(values[lowest_index] > values[j]){
                    lowest_index = j;
                }
            }
            double tmp = values[i];
            values[i] = values[lowest_index];
            values[lowest_index] = tmp;
        }
    }

    public static void test(int length){
        long start = System.nanoTime();
        double[] values = new double[length];
        for(int i = 0;i < length;i++){
            values[i] = length - i;
        }
        sort(values);
        long performance = System.nanoTime() - start;
        for(double value : values){
            System.out.println(value);
        }
        System.out.println(performance / 1000000000.0);
    }
}
