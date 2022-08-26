require 'benchmark'

class RenderMeasurement
  def measure(func)
    Benchmark.realtime do
      100.times do
        func
      end
    end
  end

  def get_data(sym) = Foo.where(render_type: sym)

  def to_times(sym)
    return def get_times(i)
      get_data(sym).first(i).render_time
    end
  end
  

  def self.exec
    100.times do
      Foo.create!(
        render_time: measure(FoosController.new.render_test_a),
        render_type: :symbol
      )
      Foo.create!(
        render_time: measure(FoosController.new.render_test_b),
        render_type: :string
      )
    end
    File.open('all_result.csv', 'w') do |f|
      sym_times,str_times = [:symbol, :string].map(&:to_times)
      f.write('計測回数,シンボル,文字列\n')
      (1..100).each do |i|
        f.write("#{i},#{sym_times(i)},#{str_times(i)}\n")
      end
    end
    File.open('result.csv', 'w') do |f|
      f.write(',シンボル,文字列\n')
      def sum(sym) = get_data(sym).sum(:render_time)
      def average(sym) = get_data(sym).average(:render_time)
      f.write("合計,#{sum :symbol},#{sum :string}")
      f.write("平均,#{average :symbol},#{average :string}")
      f.write("比(シンボルを100とした),100,#{average(:string) / average(:symbol)}")
    end
  end
end
