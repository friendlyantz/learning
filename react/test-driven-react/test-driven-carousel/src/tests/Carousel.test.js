import React from "react";
import { shallow } from "enzyme";
import Carousel from "../Carousel";
import CarouselButton from "../CarouselButton";

describe("Carousel", () => {
  let wrapper;

  beforeEach(() => {
    wrapper = shallow(<Carousel />);
  });

  it("renders a <div>", () => {
    expect(wrapper.type()).toBe("div");
  });

  it("has an initial `slideIndex` of 0", () => {
    expect(wrapper.state("slideIndex")).toBe(0);
  });

  it('renders a CarouselButton labeld "Prev"', () => {
    expect(
      wrapper
        .find(CarouselButton)
        .at(0)
        .prop("children")
    ).toBe("Prev");
  });

  it('renders a CarouselButton labeld "Next"', () => {
    expect(
      wrapper
        .find(CarouselButton)
        .at(1)
        .prop("children")
    ).toBe("Next");
  });

  it("decrements `slideIndex` when the Prev button is clicked", () => {
    wrapper.setState({ slideIndex: 1 });
    wrapper.find('[data-action="prev"]').simulate("click");
    expect(wrapper.state("slideIndex")).toBe(0);
  });

  it("increments `slideIndex` when the Next button is clicked", () => {
    wrapper.setState({ slideIndex: 1 });
    wrapper.find('[data-action="next"]').simulate("click");
    expect(wrapper.state("slideIndex")).toBe(2);
  });
});
