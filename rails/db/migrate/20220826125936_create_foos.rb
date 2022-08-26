class CreateFoos < ActiveRecord::Migration[7.0]
  def change
    create_table :foos do |t|
      t.float :render_time
      t.string :render_type

      t.timestamps
    end
  end
end
