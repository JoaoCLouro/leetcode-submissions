class Solution {
    public int[] replaceElements(int[] arr) {
        int biggest = arr[arr.length -1];
        arr[arr.length-1] = -1;
        for (int i = arr.length - 2; i >= 0; i--) {
            if (arr[i] > biggest) {
                int tmp = arr[i];
                arr[i] = biggest;
                biggest = tmp;
            }
            else {
                arr[i] = biggest;
            }
        }
        return arr;
    }
}