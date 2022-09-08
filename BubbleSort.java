// 10: 3.58E-6
// 100: 1.46964E-4
// 1000: 0.00325546
// 10000: 0.06482598
// 100000: 2.542452682
public class BubbleSort {
    public static void sort(double[] values){
        for(int i = 0;i < values.length;i++){
            for(int j = 0;j < values.length - i - 1;j++){
                if(values[j] > values[j + 1]){
                    double tmp = values[j];
                    values[j] = values[j + 1];
                    values[j + 1] = tmp;
                }
            }
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
