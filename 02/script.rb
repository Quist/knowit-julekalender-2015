prices = ARGF.read.lines.map{|price| price.to_f}

lowest_price = prices.first
highest_diff = 0

prices.each do |price|
	if (price - lowest_price) > highest_diff
		highest_diff = price - lowest_price
	end

	if price < lowest_price
		lowest_price = price
	end
end

puts "#{format("%.4f", highest_diff)}"