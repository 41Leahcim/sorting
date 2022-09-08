// 10: 3.932E-6
// 100: 2.24914E-4
// 1000: 0.003424721
// 10000: 0.108468098
// 100000: 13.120804902
public class InsertionSort {
    public static void sort(double[] values){
        int i = 1;
        while(i < values.length){
            if(values[i - 1] > values[i]){
                double tmp = values[i - 1];
                values[i - 1] = values[i];
                values[i] = tmp;
                if(i > 1){
                    i--;
                }else{
                    i++;
                }
            }else{
                i++;
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
