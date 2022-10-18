class Sort{
    //          1 : 00:00:00.0004033
    //         10 : 00:00:00.0004814
    //        100 : 00:00:00.0006029
    //       1000 : 00:00:00.0005562
    //      10000 : 00:00:00.0020149
    //     100000 : 00:00:00.0307200
    //    1000000 : 00:00:00.2200732
    //   10000000 : 00:00:02.7534234
    //  100000000 : 00:00:28.6940115
    public static void bubble_sort(double[] values){
        // 
        for(int i = 0;i < values.Length;i++){
            for(int j = 1;j < values.Length - i;j++){
                if(values[j - 1] > values[j]){
                    (values[j - 1], values[j]) = (values[j], values[j - 1]);
                }
            }
        }
    }

    //      1 : 00:00:00.0002285
    //     10 : 00:00:00.0001567
    //    100 : 00:00:00.0002073
    //   1000 : 00:00:00.0048005
    //  10000 : 00:00:00.5674889
    // 100000 : 00:01:02.1935580
    public static void insertion_sort(double[] values){
        int i = 1;
        while(i < values.Length){
            if(values[i] < values[i - 1]){
                (values[i - 1], values[i]) = (values[i], values[i - 1]);
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

    //         1 : 00:00:00.0003274
    //        10 : 00:00:00.0005750
    //       100 : 00:00:00.0005641
    //      1000 : 00:00:00.0008178
    //     10000 : 00:00:00.0018983
    //    100000 : 00:00:00.0336857
    //   1000000 : 00:00:00.2280445
    //  10000000 : 00:00:02.5885614
    // 100000000 : 00:00:28.3853749
    public static void merge_sort(double[] array){
        if(array.Length < 2){
            return;
        }else if(array.Length == 2 && array[0] > array[1]){
            (array[0], array[1]) = (array[1], array[0]);
        }else if(array.Length > 2){
            double[] array2 = new double[array.Length / 2];
            double[] array3 = new double[array.Length - array2.Length];
            int i = 0, j = 0, k = 0;
            while(i < array2.Length){
                array2[i++] = array[k++];
            }
            while(j < array3.Length){
                array3[j++] = array[k++];
            }
            merge_sort(array2);
            merge_sort(array3);
            (i, j, k) = (0, 0, 0);
            while(i < array2.Length && j < array3.Length){
                if(array2[i] < array3[j]){
                    array[k++] = array2[i++];
                }else{
                    array[k++] = array3[j++];
                }
            }
            while(i < array2.Length){
                array[k++] = array2[i++];
            }
            while(j < array3.Length){
                array[k++] = array3[j++];
            }
        }
    }

    //      1 : 00:00:00.0001463
    //     10 : 00:00:00.0001672
    //    100 : 00:00:00.0001966
    //   1000 : 00:00:00.0016945
    //  10000 : 00:00:00.1700944
    // 100000 : 00:00:18.5099241
    public static void selection_sort(double[] values){
        for(int i = 0;i < values.Length;i++){
            int lowest_index = i;
            for(int j = i + 1;j < values.Length;j++){
                if(values[j] < values[lowest_index]){
                    lowest_index = j;
                }
            }
            (values[i], values[lowest_index]) = (values[lowest_index], values[i]);
        }
    }
};