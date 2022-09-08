// 10: 5.093E-6
// 100: 3.6858E-5
// 1000: 4.31988E-4
// 10000: 0.001746159
// 100000: 0.055399026
// 1000000: 0.089316221
// 10000000: 0.839243773
// 100000000: 7.356650992
public class MergeSort {
    public static double[] sort(double[] values){
        if(values.length == 2 && values[0] > values[1]){
            double tmp = values[0];
            values[0] = values[1];
            values[1] = tmp;
        }else if(values.length > 2){
            double[] values2 = new double[values.length / 2];
            double[] values3 = new double[values.length - values2.length];
            for(int i = 0;i < values2.length;i++){
                values2[i] = values[i];
            }
            for(int i = 0;i < values3.length;i++){
                values3[i] = values[values2.length + i];
            }
            sort(values2);
            sort(values3);
            int i = 0, j = 0, k = 0;
            while(i < values2.length && j < values3.length){
                if(values2[i] < values3[j]){
                    values[k++] = values2[i++];
                }else{
                    values[k++] = values3[j++];
                }
            }
            while(i < values2.length){
                values[k++] = values2[i++];
            }
            while(j < values3.length){
                values[k++] = values3[i++];
            }
        }
        return values;
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
