export function maintain_aspect_ratio(aspectRatio, id) {
  let container = document.getElementById(id);
  let child = container.children[0];

  container.style.overflow = "hidden";

  child.style.aspectRatio = aspectRatio;
  child.style.margin = "0";

  let on_resize = () => {
    let width = container.clientWidth;
    let height = container.clientHeight;

    let container_aspect_ratio = width / height;

    if (container_aspect_ratio > aspectRatio) {
      child.style.width = "unset";
      child.style.height = "100%";
    } else {
      child.style.width = "100%";
      child.style.height = "unset";
    }
  };

  new ResizeObserver(on_resize).observe(container);
}
