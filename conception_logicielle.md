 1. One-time setup
    Parsing
        is_valid_file(): verify if the .obj file exist and can be read 
        parse_obj_file(): read file line to line
            extract_vertices(): Get vertices (v)
            extract_faces() :Get faces idx (f) (faces are the triangles for draw)
        build_mesh() : regroup data

    Context
        init_window(): make window
        init_vulkan(): Init connexion with gpu and send him mesh one time

2. Render Loop
    Inputs
        poll_events(): listen keyboard and mouse
            handle_movement(): change position and rotation variable depending on which key is pressed (3d moving)
            handle_texture_toggle(): update texture variable

    logic update
        compute_matrices() : 3D calculation
            update_model_matrix(): symmetry axial rotation Apply
            update_view_matrix(): camera issue (zoom, translations)
            update_projection_matrix(): perspective effect Apply

    Draw call
        send_uniforms_to_gpu(): send new matrix and texture level transition to vulkan
      draw_frame() : Ask to vulkan to calculate the final image and display it in the window
