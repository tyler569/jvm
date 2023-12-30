public class Apple {
    public int seeds = 10;
    private int juice = 11;

    public int getJuice() {
        return juice;
    }

    public void setJuice(int newJuice) {
        juice = newJuice;
    }

    public void bite() {
        juice -= 1;
        System.out.println("You took a bite!");
    }
}
