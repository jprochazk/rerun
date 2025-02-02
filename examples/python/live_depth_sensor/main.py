#!/usr/bin/env python3
"""A minimal example of streaming frames live from an Intel RealSense depth sensor."""
from __future__ import annotations

import argparse

import numpy as np
import pyrealsense2 as rs
import rerun as rr  # pip install rerun-sdk


def run_realsense(num_frames: int | None) -> None:
    # Visualize the data as RDF
    rr.log_view_coordinates("realsense", xyz="RDF", timeless=True)

    # Open the pipe
    pipe = rs.pipeline()
    profile = pipe.start()

    # We don't log the depth exstrinsics. We treat the "realsense" space as being at
    # the origin of the depth sensor so that "realsense/depth" = Identity

    # Get and log depth intrinsics
    depth_profile = profile.get_stream(rs.stream.depth)
    depth_intr = depth_profile.as_video_stream_profile().get_intrinsics()

    rr.log_pinhole(
        "realsense/depth/image",
        width=depth_intr.width,
        height=depth_intr.height,
        focal_length_px=[depth_intr.fx, depth_intr.fy],
        principal_point_px=[depth_intr.ppx, depth_intr.ppy],
        timeless=True,
    )

    # Get and log color extrinsics
    rgb_profile = profile.get_stream(rs.stream.color)

    rgb_from_depth = depth_profile.get_extrinsics_to(rgb_profile)
    rr.log_transform3d(
        "realsense/rgb",
        transform=rr.TranslationAndMat3(
            translation=rgb_from_depth.translation, matrix=np.reshape(rgb_from_depth.rotation, (3, 3))
        ),
        from_parent=True,
        timeless=True,
    )

    # Get and log color intrinsics
    rgb_intr = rgb_profile.as_video_stream_profile().get_intrinsics()

    rr.log_pinhole(
        "realsense/rgb/image",
        width=rgb_intr.width,
        height=rgb_intr.height,
        focal_length_px=[rgb_intr.fx, rgb_intr.fy],
        principal_point_px=[rgb_intr.ppx, rgb_intr.ppy],
        timeless=True,
    )

    # Read frames in a loop
    frame_nr = 0
    try:
        while True:
            if num_frames and frame_nr >= num_frames:
                break

            rr.set_time_sequence("frame_nr", frame_nr)
            frame_nr += 1

            frames = pipe.wait_for_frames()
            for f in frames:
                # Log the depth frame
                depth_frame = frames.get_depth_frame()
                depth_units = depth_frame.get_units()
                depth_image = np.asanyarray(depth_frame.get_data())
                rr.log_depth_image("realsense/depth/image", depth_image, meter=1.0 / depth_units)

                # Log the color frame
                color_frame = frames.get_color_frame()
                color_image = np.asanyarray(color_frame.get_data())
                rr.log_image("realsense/rgb/image", color_image)
    finally:
        pipe.stop()


def main() -> None:
    parser = argparse.ArgumentParser(description="Streams frames from a connected realsense depth sensor.")
    parser.add_argument("--num-frames", type=int, default=None, help="The number of frames to log")

    rr.script_add_args(parser)
    args = parser.parse_args()

    rr.script_setup(args, "rerun_example_live_depth_sensor")

    run_realsense(args.num_frames)

    rr.script_teardown(args)


if __name__ == "__main__":
    main()
