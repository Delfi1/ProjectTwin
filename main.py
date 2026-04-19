import json
import os
import shutil
import subprocess
import sys

import cv2 as cv

FPS = 4


def extract_video(output_dir: str, file: str) -> int:
    video_capture = cv.VideoCapture(file)
    frame_rate = video_capture.get(cv.CAP_PROP_FPS)

    frame_interval = int(frame_rate / FPS)

    frame_count = 0
    result = 0
    while True:
        success, frame = video_capture.read()

        if not success:
            break

        if frame_count % frame_interval == 0:
            frame_filename = os.path.join(
                output_dir, f"frame_{frame_count // frame_interval}.jpg"
            )

            result += 1
            cv.imwrite(frame_filename, frame)

        frame_count += 1

    return result


# Obj - object name and dir name
def read_timeline(obj: str):
    # Read all video clips
    t = 0
    while True:
        destination_dir = rf".\\instant-ngp\\data\\nerf\\{obj}\\t{t}"
        folder = f".\\{obj}\\t{t}"

        if os.path.exists(destination_dir):
            t += 1
            continue

        if os.path.isdir(folder):
            files = os.listdir(folder)

            images_path = rf"{destination_dir}\\images"
            if not os.path.exists(images_path):
                os.makedirs(images_path)

            for f in files:
                shutil.copy(os.path.join(folder, f), images_path)

            print(f"------ Extracting files from {folder} ------")
        else:
            file = f".\\{obj}\\t{t}.mp4"
            if not os.path.exists(file):
                break

            print(f"------ Extracting {file} ------")

            images_path = rf"{destination_dir}\\images"
            if not os.path.exists(images_path):
                os.makedirs(images_path)

            extract_video(images_path, file)

        input("\n------ Create colmap? ------")

        colmap2nerf_path = r".\\instant-ngp\\scripts\\colmap2nerf.py"
        colmap2nerf_arguments = [
            "--colmap_matcher",
            "exhaustive",
            "--run_colmap",
            "--aabb_scale",
            "16",
            "--images",
            images_path,
            "--overwrite",
        ]

        command = ["python", colmap2nerf_path] + colmap2nerf_arguments
        subprocess.run(command)

        if not os.path.exists(destination_dir):
            os.makedirs(destination_dir)

        with open("transforms.json", "r") as file:
            data = json.load(file)

        with open("transforms.json", "w") as file:
            for f in data["frames"]:
                i = f["file_path"].rfind("images")
                f["file_path"] = f["file_path"][i:]

            json.dump(data, file)

        shutil.move("transforms.json", destination_dir)

        subprocess.run(
            f".\\instant-ngp.exe \\data\\nerf\\{obj}\\t{t}", cwd="instant-ngp"
        )

        a = input("Restart?")
        if a.lower() == "y":
            shutil.rmtree(destination_dir)
            continue

        t += 1


if __name__ == "__main__":
    read_timeline(sys.argv[1])
