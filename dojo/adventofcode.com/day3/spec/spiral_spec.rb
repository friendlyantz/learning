require 'spec_helper'
require 'spiral'

RSpec.describe Spiral do
  {
    1 => [[1]],
    2 => [[0,0,0],[0,1,2],[0,0,0]],
    3 => [[0,0,3],[0,1,2],[0,0,0]],
    4 => [[0,4,3],[0,1,2],[0,0,0]],
    5 => [[5,4,3],[0,1,2],[0,0,0]],
    6 => [[5,4,3],[6,1,2],[0,0,0]],
    7 => [[5,4,3],[6,1,2],[7,0,0]],
    8 => [[5,4,3],[6,1,2],[7,8,0]],
    9 => [[5,4,3],[6,1,2],[7,8,9]],
    10 => [
      [0,0,0,0, 0],
      [0,5,4,3, 0],
      [0,6,1,2, 0],
      [0,7,8,9,10],
      [0,0,0,0, 0]
    ],
    25 => [
      [17,16,15,14,13],
      [18, 5, 4, 3,12],
      [19, 6, 1, 2,11],
      [20, 7, 8, 9,10],
      [21,22,23,24,25]
    ],
    #4 => [[4, 3],[1, 2]],
  }.each do |limit, square|

    context "limit #{limit} to #{square}" do
      subject(:spiral) { Spiral.new(limit) }
      it { expect(spiral.show).to eq(square) }
    end
  end
end
